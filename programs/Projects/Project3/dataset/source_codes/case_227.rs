use anchor_lang::prelude::*;
use anchor_lang::sysvar::clock::Clock;

// Program ID - replace with your own
declare_id!("Fg6PaFpoGXkYsidMpG1H2J3K4L5M6N7O8P9Q0R1S2T3");

#[program]
pub mod analytics {
    use super::*;

    /// アナリティクスアカウントを初期化
    pub fn initialize(
        ctx: Context<InitializeAnalytics>,
        bump: u8,
    ) -> ProgramResult {
        let stats = &mut ctx.accounts.analytics;
        stats.owner = *ctx.accounts.user.key;
        stats.bump = bump;
        stats.login_count = 0;
        stats.action_count = 0;
        stats.last_login = 0;
        stats.last_action = 0;
        Ok(())
    }

    /// ログインイベントを記録
    pub fn record_login(
        ctx: Context<RecordLogin>,
    ) -> ProgramResult {
        let stats = &mut ctx.accounts.analytics;
        stats.login_count = stats.login_count.wrapping_add(1);
        stats.last_login = Clock::get()?.unix_timestamp;
        Ok(())
    }

    /// ユーザーアクションを記録
    pub fn record_action(
        ctx: Context<RecordAction>,
        actions: u64,
    ) -> ProgramResult {
        let stats = &mut ctx.accounts.analytics;
        stats.action_count = stats.action_count.wrapping_add(actions);
        stats.last_action = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

/// 初期化用コンテキスト
#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeAnalytics<'info> {
    #[account(
        init,
        seeds = [b"analytics", user.key().as_ref()],
        bump = bump,
        payer = user,
        space = 8 + 32 + 1 + 8 + 8 + 8 + 8,
    )]
    pub analytics: Account<'info, Analytics>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

/// ログイン記録用コンテキスト
#[derive(Accounts)]
pub struct RecordLogin<'info> {
    #[account(
        mut,
        seeds = [b"analytics", analytics.owner.as_ref()],
        bump = analytics.bump,
        has_one = owner,
    )]
    pub analytics: Account<'info, Analytics>,
    pub owner: Signer<'info>,
}

/// アクション記録用コンテキスト
#[derive(Accounts)]
pub struct RecordAction<'info> {
    #[account(
        mut,
        seeds = [b"analytics", analytics.owner.as_ref()],
        bump = analytics.bump,
        has_one = owner,
    )]
    pub analytics: Account<'info, Analytics>,
    pub owner: Signer<'info>,
}

/// アナリティクスデータ構造
#[account]
pub struct Analytics {
    pub owner: Pubkey,
    pub bump: u8,
    pub login_count: u64,
    pub action_count: u64,
    pub last_login: i64,
    pub last_action: i64,
}

// 分岐やループを含まず、多様なフィールドを安全に扱う実装です。
