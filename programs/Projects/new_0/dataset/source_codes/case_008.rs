use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjf777mvTWf");

#[program]
pub mod time_lock_003 {
    use super::*;

    // 解禁時間を設定
    pub fn set_unlock_time(ctx: Context<Ctx003>, unlock_unix_time: u64) -> Result<()> {
        require!(ctx.accounts.authority.is_signer, CustomError::Unauthorized);
        ctx.accounts.storage.data = unlock_unix_time;
        msg!("Unlock time set to {}", unlock_unix_time);
        Ok(())
    }

    // 現在の時刻が解禁時間を超えていれば実行可能な処理
    pub fn perform_unlocked_action(ctx: Context<Ctx003>) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp as u64;
        let unlock_time = ctx.accounts.storage.data;

        require!(
            current_time >= unlock_time,
            CustomError::NotYetUnlocked
        );

        msg!(
            "Action permitted. Current: {}, Unlock: {}",
            current_time,
            unlock_time
        );
        Ok(())
    }

    // 現在の時刻とロック状態を表示
    pub fn check_status(ctx: Context<Ctx003>) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp as u64;
        let unlock_time = ctx.accounts.storage.data;

        let status = if current_time >= unlock_time {
            "UNLOCKED"
        } else {
            "LOCKED"
        };

        msg!(
            "Status: {} | Now: {}, Unlock Time: {}",
            status,
            current_time,
            unlock_time
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx003<'info> {
    #[account(mut, has_one = authority)]
    pub storage: Account<'info, Storage003>,
    #[account(signer)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Storage003 {
    pub authority: Pubkey,
    pub data: u64, // UNIX時刻（ロック解除時刻）
}

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Time lock still active")]
    NotYetUnlocked,
}
