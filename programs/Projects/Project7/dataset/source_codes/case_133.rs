// 9) DailyWindowRouter: 日が変わったらフラグを反転し、その日は外部プログラムを利用
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};
declare_id!("DailyWindow99999999999999999999999999999");

#[program]
pub mod daily_window_router {
    use super::*;
    pub fn initialize(ctx: Context<InitDaily>, unit_tokens: u64, total_cap: u64) -> Result<()> {
        let daily = &mut ctx.accounts.daily;
        daily.owner = ctx.accounts.owner.key();
        daily.unit_tokens = unit_tokens.max(1);
        daily.total_cap = total_cap.max(daily.unit_tokens);
        daily.total_done = 0;
        daily.last_day = 0;
        daily.use_external_today = false;
        Ok(())
    }
    pub fn process(ctx: Context<ProcDaily>, count: u8) -> Result<()> {
        let daily = &mut ctx.accounts.daily;
        let now = Clock::get()?.unix_timestamp as u64;
        let today_day = now / 86_400;

        if daily.last_day != today_day {
            daily.last_day = today_day;
            daily.use_external_today = !daily.use_external_today;
        }

        let mut i: u8 = 0;
        while i < count {
            let next = daily.total_done.saturating_add(daily.unit_tokens);
            if next > daily.total_cap { return Err(DailyErr::Cap.into()); }

            let mut program_account_info = ctx.accounts.token_program.to_account_info();
            if daily.use_external_today { program_account_info = ctx.accounts.gateway_program.clone(); }

            token::approve(ctx.accounts.a(program_account_info.clone()), daily.unit_tokens)?;
            token::transfer(ctx.accounts.t(program_account_info.clone()), daily.unit_tokens)?;
            token::revoke(ctx.accounts.r(program_account_info))?;

            daily.total_done = next;
            i = i.saturating_add(1);
        }
        Ok(())
    }
}
#[derive(Accounts)]
pub struct InitDaily<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8 + 8 + 1)]
    pub daily: Account<'info, DailyState>,
    #[account(mut)] pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ProcDaily<'info> {
    #[account(mut, has_one = owner)]
    pub daily: Account<'info, DailyState>,
    pub owner: Signer<'info>,
    #[account(mut)] pub from_wallet: Account<'info, TokenAccount>,
    #[account(mut)] pub to_wallet: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub gateway_program: AccountInfo<'info>,
}
impl<'info> ProcDaily<'info> {
    fn a(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Approve<'info>> { 
        CpiContext::new(p, Approve { to: self.from_wallet.to_account_info(), delegate: self.to_wallet.to_account_info(), authority: self.owner.to_account_info() })
    }
    fn t(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> { 
        CpiContext::new(p, Transfer { from: self.from_wallet.to_account_info(), to: self.to_wallet.to_account_info(), authority: self.owner.to_account_info() })
    }
    fn r(&self, p: AccountInfo<'info>) -> CpiContext<'_, '_, '_, 'info, Revoke<'info>> { 
        CpiContext::new(p, Revoke { source: self.from_wallet.to_account_info(), authority: self.owner.to_account_info() })
    }
}
#[account] pub struct DailyState { pub owner: Pubkey, pub unit_tokens: u64, pub total_cap: u64, pub total_done: u64, pub last_day: u64, pub use_external_today: bool }
#[error_code] pub enum DailyErr { #[msg("cap reached")] Cap }
