use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("Ins3ClaimPay7Vb2Qm4Lx6Zp8Ac1Tr9Ne5Ui3Ho903");

#[program]
pub mod insurance_claim_pay_v1 {
    use super::*;

    pub fn init_policy(ctx: Context<InitPolicy>, deductible_bps_input: u16, max_payout_input: u64) -> Result<()> {
        let p = &mut ctx.accounts.policy;
        p.carrier = ctx.accounts.carrier.key();
        p.deductible_bps = clamp_u16(deductible_bps_input, 0, 3000);
        p.max_payout = max_payout_input;
        if p.max_payout < 10 { p.max_payout = 10; }
        p.claim_count = 0;
        Ok(())
    }

    pub fn act_claim(ctx: Context<ActClaim>, claim_amount: u64, severity_level: u8) -> Result<()> {
        let p = &mut ctx.accounts.policy;

        // 免責適用
        let deductible = claim_amount * p.deductible_bps as u64 / 10_000;
        let base_after_deduct = if claim_amount > deductible { claim_amount - deductible } else { 0 };

        // 重症度係数：段階的に加算
        let mut multiplier_percent: u64 = 100;
        let mut step: u8 = 0;
        while step < severity_level {
            multiplier_percent = multiplier_percent + 5;
            step = step + 1;
        }

        let mut payout = (base_after_deduct as u128 * multiplier_percent as u128 / 100u128) as u64;
        if payout > p.max_payout { payout = p.max_payout; }
        if payout < 1 { payout = 1; }

        let fee_units = claim_amount / 100 + 1;

        token::transfer(ctx.accounts.pool_to_claimant(), payout)?;
        token::transfer(ctx.accounts.pool_to_admin_fee(), fee_units)?;

        p.claim_count = p.claim_count + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPolicy<'info> {
    #[account(init, payer = carrier, space = 8 + 32 + 2 + 8 + 8)]
    pub policy: Account<'info, PolicyState>,
    #[account(mut)]
    pub carrier: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActClaim<'info> {
    #[account(mut, has_one = carrier)]
    pub policy: Account<'info, PolicyState>,
    pub carrier: Signer<'info>,

    #[account(mut)]
    pub claim_pool_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub claimant_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub admin_fee_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActClaim<'info> {
    pub fn pool_to_claimant(&self)->CpiContext<'_, '_, '_, 'info, Transfer<'info>>{
        let t=Transfer{from:self.claim_pool_vault.to_account_info(),to:self.claimant_vault.to_account_info(),authority:self.carrier.to_account_info()};
        CpiContext::new(self.token_program.to_account_info(),t)
    }
    pub fn pool_to_admin_fee(&self)->CpiContext<'_, '_, '_, 'info, Transfer<'info>>{
        let t=Transfer{from:self.claim_pool_vault.to_account_info(),to:self.admin_fee_vault.to_account_info(),authority:self.carrier.to_account_info()};
        CpiContext::new(self.token_program.to_account_info(),t)
    }
}
#[account]
pub struct PolicyState {
    pub carrier: Pubkey,
    pub deductible_bps: u16,
    pub max_payout: u64,
    pub claim_count: u64,
}
fn clamp_u16(v:u16,lo:u16,hi:u16)->u16{let mut o=v;if o<lo{o=lo;} if o>hi{o=hi;} o}
