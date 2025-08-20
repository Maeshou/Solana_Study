// (3) DailyExternalToggle: 日替わりで program を切り替え
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Transfer, Revoke, Token, TokenAccount};

declare_id!("DailyExternal3333333333333333333333333333");

#[program]
pub mod daily_external_toggle {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, cap_limit: u64) -> Result<()> {
        let daily_state = &mut ctx.accounts.daily_state;
        daily_state.admin = ctx.accounts.admin.key();
        daily_state.cap_limit = cap_limit;
        daily_state.total_moved = 0;
        daily_state.last_epoch_day = 0;
        Ok(())
    }

    pub fn run(ctx: Context<Run>, move_amount: u64) -> Result<()> {
        let daily_state = &mut ctx.accounts.daily_state;
        let current_day = (Clock::get()?.unix_timestamp as u64) / 86_400;

        let mut program_handle = ctx.accounts.token_program.to_account_info();
        if daily_state.last_epoch_day != current_day {
            program_handle = ctx.accounts.external_program.clone(); // 切替
            daily_state.last_epoch_day = current_day;
        }

        let next_total = daily_state.total_moved.saturating_add(move_amount);
        require!(next_total <= daily_state.cap_limit, DailyErr::CapReached);

        token::approve(CpiContext::new(program_handle.clone(), Approve {
            to: ctx.accounts.source_tokens.to_account_info(),
            delegate: ctx.accounts.destination_tokens.to_account_info(),
            authority: ctx.accounts.admin.to_account_info(),
        }), move_amount)?;
        token::transfer(CpiContext::new(program_handle.clone(), Transfer {
            from: ctx.accounts.source_tokens.to_account_info(),
            to: ctx.accounts.destination_tokens.to_account_info(),
            authority: ctx.accounts.admin.to_account_info(),
        }), move_amount)?;
        token::revoke(CpiContext::new(program_handle, Revoke {
            source: ctx.accounts.source_tokens.to_account_info(),
            authority: ctx.accounts.admin.to_account_info(),
        }))?;

        daily_state.total_moved = next_total;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 8 + 8)]
    pub daily_state: Account<'info, DailyState>,
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Run<'info> {
    #[account(mut, has_one = admin)]
    pub daily_state: Account<'info, DailyState>,
    pub admin: Signer<'info>,
    #[account(mut)] pub source_tokens: Account<'info, TokenAccount>,
    #[account(mut)] pub destination_tokens: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub external_program: AccountInfo<'info>,
}

#[account] pub struct DailyState { pub admin: Pubkey, pub cap_limit: u64, pub total_moved: u64, pub last_epoch_day: u64 }
#[error_code] pub enum DailyErr { #[msg("cap reached for the day")] CapReached }
