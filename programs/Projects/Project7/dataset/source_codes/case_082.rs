use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};

declare_id!("Bike6TopUp7Kc2Qm4Lx6Zp8Vt1Na3Ur5Hs9Kd906");

#[program]
pub mod bike_share_topup_v1 {
    use super::*;

    pub fn init_station(ctx: Context<InitStation>, chunk_input: u64, daily_limit_input: u64) -> Result<()> {
        let s = &mut ctx.accounts.station;
        s.manager = ctx.accounts.manager.key();
        s.chunk = chunk_input;
        if s.chunk < 2 { s.chunk = 2; }
        s.daily_limit = daily_limit_input;
        if s.daily_limit < s.chunk { s.daily_limit = s.chunk; }
        s.sent_today = 0;
        s.cool_grade = 1;
        Ok(())
    }

    pub fn act_topup(ctx: Context<ActTopup>, bursts: u8) -> Result<()> {
        let s = &mut ctx.accounts.station;

        let mut i: u8 = 0;
        while i < bursts {
            let divider = s.cool_grade + 1;
            let mut amount = s.chunk / divider;
            if amount < 1 { amount = 1; }

            let next = s.sent_today + amount;
            if next > s.daily_limit {
                s.cool_grade = s.cool_grade + 1;
                return Err(TopupErr::Daily.into());
            }

            token::approve(ctx.accounts.approve_ctx(), amount)?;
            token::transfer(ctx.accounts.transfer_ctx(), amount)?;
            token::revoke(ctx.accounts.revoke_ctx())?;

            s.sent_today = next;

            let relief_step = s.chunk * 4;
            if relief_step > 0 {
                if s.sent_today % relief_step == 0 {
                    if s.cool_grade > 0 { s.cool_grade = s.cool_grade - 1; }
                }
            }
            i = i + 1;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStation<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 8 + 8 + 8 + 8)]
    pub station: Account<'info, BikeStation>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActTopup<'info> {
    #[account(mut, has_one = manager)]
    pub station: Account<'info, BikeStation>,
    pub manager: Signer<'info>,

    #[account(mut)]
    pub payment_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_wallet_vault: Account<'info, TokenAccount>,
    /// CHECK: 任意委任先
    pub delegate: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActTopup<'info> {
    pub fn approve_ctx(&self)->CpiContext<'_, '_, '_, 'info, Approve<'info>>{
        let a=Approve{to:self.payment_vault.to_account_info(),delegate:self.delegate.to_account_info(),authority:self.manager.to_account_info()};
        CpiContext::new(self.token_program.to_account_info(),a)
    }
    pub fn transfer_ctx(&self)->CpiContext<'_, '_, '_, 'info, Transfer<'info>>{
        let t=Transfer{from:self.payment_vault.to_account_info(),to:self.user_wallet_vault.to_account_info(),authority:self.manager.to_account_info()};
        CpiContext::new(self.token_program.to_account_info(),t)
    }
    pub fn revoke_ctx(&self)->CpiContext<'_, '_, '_, 'info, Revoke<'info>>{
        let r=Revoke{source:self.payment_vault.to_account_info(),authority:self.manager.to_account_info()};
        CpiContext::new(self.token_program.to_account_info(),r)
    }
}
#[account]
pub struct BikeStation {
    pub manager: Pubkey,
    pub chunk: u64,
    pub daily_limit: u64,
    pub sent_today: u64,
    pub cool_grade: u64,
}
#[error_code]
pub enum TopupErr { #[msg("daily limit reached")] Daily }
