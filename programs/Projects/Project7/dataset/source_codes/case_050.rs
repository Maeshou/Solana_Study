use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("ReNt4lEscrowP7A2Y9M5Q1L6X8Z3V0C4T2R404");

#[program]
pub mod rental_escrow_v1 {
    use super::*;

    pub fn init_escrow(ctx: Context<InitEscrow>, fee_bps: u16, deposit_units: u64) -> Result<()> {
        let es = &mut ctx.accounts.escrow;
        es.arbitrator = ctx.accounts.arbitrator.key();
        es.fee_bps = clamp_u16(fee_bps, 0, 2000);
        es.deposit_required = if deposit_units < 5 { 5 } else { deposit_units };
        es.days_recorded = 1;
        es.total_released = 2;
        Ok(())
    }

    pub fn act_release(ctx: Context<ActRelease>, rent_days: u8, disputed: bool) -> Result<()> {
        let es = &mut ctx.accounts.escrow;

        // 日数に応じて家賃総額
        let mut total_rent = es.deposit_required;
        let mut d: u8 = 1;
        while d < rent_days {
            let mut per = es.deposit_required / 3;
            if d >= 5 { per = per + per / 5; }
            total_rent = total_rent + per;
            d = d + 1;
        }

        // 手数料
        let fee = total_rent * es.fee_bps as u64 / 10_000;

        // 分岐：紛争時は半分返金、通常は全額支払
        if disputed {
            let refund = total_rent / 2;
            token::transfer(ctx.accounts.escrow_to_renter(), refund)?;
            token::transfer(ctx.accounts.escrow_to_fee(), fee)?;
            es.total_released = es.total_released + refund;
        }
        if !disputed {
            let payout = total_rent - fee;
            token::transfer(ctx.accounts.escrow_to_landlord(), payout)?;
            token::transfer(ctx.accounts.escrow_to_fee(), fee)?;
            es.total_released = es.total_released + payout;
        }

        es.days_recorded = es.days_recorded + rent_days as u64;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitEscrow<'info> {
    #[account(init, payer = arbitrator, space = 8 + 32 + 2 + 8 + 8 + 8)]
    pub escrow: Account<'info, EscrowState>,
    #[account(mut)]
    pub arbitrator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActRelease<'info> {
    #[account(mut, has_one = arbitrator)]
    pub escrow: Account<'info, EscrowState>,
    pub arbitrator: Signer<'info>,

    #[account(mut)]
    pub escrow_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub landlord_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub renter_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub fee_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActRelease<'info> {
    pub fn escrow_to_landlord(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.escrow_vault.to_account_info(), to: self.landlord_vault.to_account_info(), authority: self.arbitrator.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), t)
    }
    pub fn escrow_to_renter(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.escrow_vault.to_account_info(), to: self.renter_vault.to_account_info(), authority: self.arbitrator.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), t)
    }
    pub fn escrow_to_fee(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer { from: self.escrow_vault.to_account_info(), to: self.fee_vault.to_account_info(), authority: self.arbitrator.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), t)
    }
}

#[account]
pub struct EscrowState {
    pub arbitrator: Pubkey,
    pub fee_bps: u16,
    pub deposit_required: u64,
    pub days_recorded: u64,
    pub total_released: u64,
}

fn clamp_u16(v: u16, lo: u16, hi: u16) -> u16 { let mut o=v; if o<lo{ o=lo; } if o>hi{ o=hi; } o }
