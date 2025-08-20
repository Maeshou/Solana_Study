use anchor_lang::prelude::*;
use anchor_lang::sysvar::clock::Clock;

// Program ID - replace with your own
declare_id!("Fg6PaFpoGXkYsidMpE5F6G7H8J9K0L1M2N3O4P5Q6R7S8");

#[program]
pub mod timestamp_logger {
    use super::*;

    /// ログアカウントの初期化
    pub fn initialize(ctx: Context<InitializeLogger>, bump: u8) -> ProgramResult {
        let logger = &mut ctx.accounts.logger;
        logger.owner = *ctx.accounts.user.key;
        logger.bump = bump;
        logger.last_timestamp = 0;
        Ok(())
    }

    /// 現在の UNIX タイムスタンプを記録
    pub fn log(ctx: Context<LogTimestamp>) -> ProgramResult {
        let logger = &mut ctx.accounts.logger;
        logger.last_timestamp = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeLogger<'info> {
    #[account(
        init,
        seeds = [b"logger", user.key().as_ref()],
        bump = bump,
        payer = user,
        space = 8 + 32 + 1 + 8,
    )]
    pub logger: Account<'info, Logger>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct LogTimestamp<'info> {
    #[account(
        mut,
        seeds = [b"logger", logger.owner.as_ref()],
        bump = logger.bump,
        has_one = owner,
    )]
    pub logger: Account<'info, Logger>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Logger {
    /// ログ更新を許可された所有者
    pub owner: Pubkey,
    /// PDA生成用バンプ
    pub bump: u8,
    /// 最終ログ日時 (UNIX タイムスタンプ)
    pub last_timestamp: i64,
}

// 分岐・ループ不要のシンプル実装です。
