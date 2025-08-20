// 5) 日ごとに Token と System を切替（状態保持）
use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer as SysTransfer};
use anchor_spl::token::{self, Transfer as SplTransfer, Token, TokenAccount};

declare_id!("AcctOnlyDailySwitchDDDDDDDDDDDDDDDDDDDDDDD");

#[program]
pub mod daily_switch {
    use super::*;
    pub fn init(ctx: Context<InitDaily>, unit_lamports: u64, unit_tokens: u64) -> Result<()> {
        let d = &mut ctx.accounts.daily;
        d.admin = ctx.accounts.admin.key();
        d.unit_lamports = unit_lamports;
        d.unit_tokens = unit_tokens.max(1);
        d.last_day = 0;
        d.use_system_today = false;
        Ok(())
    }

    pub fn tick(ctx: Context<TickDaily>) -> Result<()> {
        let d = &mut ctx.accounts.daily;
        let now = Clock::get()?.unix_timestamp as u64;
        let today = now / 86_400;
        if d.last_day != today { d.last_day = today; d.use_system_today = !d.use_system_today; }

        if d.use_system_today {
            system_program::transfer(CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                SysTransfer {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.receiver.to_account_info(),
                },
            ), d.unit_lamports)?;
        }
        if !d.use_system_today {
            token::transfer(CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                SplTransfer {
                    from: ctx.accounts.src.to_account_info(),
                    to: ctx.accounts.dst.to_account_info(),
                    authority: ctx.accounts.admin.to_account_info(),
                },
            ), d.unit_tokens)?;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitDaily<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 8 + 8 + 1)]
    pub daily: Account<'info, DailyRoute>,
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct TickDaily<'info> {
    #[account(mut, has_one = admin)]
    pub daily: Account<'info, DailyRoute>,
    pub admin: Signer<'info>,
    // system
    #[account(mut)] pub payer: Signer<'info>,
    #[account(mut)] pub receiver: Account<'info, SystemAccount>,
    pub system_program: Program<'info, System>,
    // token
    #[account(mut)] pub src: Account<'info, TokenAccount>,
    #[account(mut)] pub dst: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
#[account] pub struct DailyRoute { pub admin: Pubkey, pub unit_lamports: u64, pub unit_tokens: u64, pub last_day: u64, pub use_system_today: bool }
