use anchor_lang::prelude::*;
use anchor_lang::sysvar::clock::Clock;

// Program ID - replace with your own
declare_id!("Fg6PaFpoGXkYsidMpE9F8G7H6J5K4L3M2N1O0P9Q8R7S6");

#[program]
pub mod event_tracker {
    use super::*;

    /// トラッカーアカウントを初期化
    pub fn initialize(
        ctx: Context<InitializeTracker>,
        bump: u8,
        initial_event_type: u8,
    ) -> ProgramResult {
        let tracker = &mut ctx.accounts.tracker;
        tracker.owner = *ctx.accounts.user.key;
        tracker.bump = bump;
        tracker.last_timestamp = 0;
        tracker.event_count = 0;
        tracker.last_event_type = initial_event_type;
        Ok(())
    }

    /// イベントを記録：タイムスタンプ、タイプ、カウントを更新
    pub fn record(
        ctx: Context<RecordEvent>,
        event_type: u8,
    ) -> ProgramResult {
        let tracker = &mut ctx.accounts.tracker;
        tracker.last_timestamp = Clock::get()?.unix_timestamp;
        tracker.last_event_type = event_type;
        tracker.event_count = tracker.event_count.wrapping_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeTracker<'info> {
    #[account(
        init,
        seeds = [b"tracker", user.key().as_ref()],
        bump = bump,
        payer = user,
        space = 8 + 32 + 1 + 8 + 8 + 1,
    )]
    pub tracker: Account<'info, Tracker>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct RecordEvent<'info> {
    #[account(
        mut,
        seeds = [b"tracker", tracker.owner.as_ref()],
        bump = tracker.bump,
        has_one = owner,
    )]
    pub tracker: Account<'info, Tracker>,
    /// イベント記録を実行する所有者
    pub owner: Signer<'info>,
}

#[account]
pub struct Tracker {
    /// アカウント所有者
    pub owner: Pubkey,
    /// PDA生成用バンプ
    pub bump: u8,
    /// 最終更新タイムスタンプ (UNIX秒)
    pub last_timestamp: i64,
    /// 記録されたイベント数
    pub event_count: u64,
    /// 最後に記録されたイベントタイプ
    pub last_event_type: u8,
}

// 分岐やループを含まず、複数のフィールドを扱う安全な実装です。
