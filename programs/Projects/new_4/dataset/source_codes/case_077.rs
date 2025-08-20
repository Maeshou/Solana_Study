use anchor_lang::prelude::*;

declare_id!("NextCaseEx20202020202020202020202020202020");

#[program]
pub mod example2 {
    use super::*;

    // イベントを作成（event にだけ init）
    pub fn schedule_event(ctx: Context<ScheduleEvent>, timestamp: i64) -> Result<()> {
        let event = &mut ctx.accounts.event;           // ← initあり
        event.when = timestamp;

        let reminder = &mut ctx.accounts.reminder;     // ← initなし（本来は初期化すべき）
        // イベントが未来ならリマインダー設定
        if timestamp > Clock::get()?.unix_timestamp {
            reminder.active = true;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ScheduleEvent<'info> {
    #[account(init, payer = organizer, space = 8 + 8)]
    pub event: Account<'info, EventData>,
    pub reminder: Account<'info, ReminderData>,
    #[account(mut)] pub organizer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct EventData {
    pub when: i64,
}

#[account]
pub struct ReminderData {
    pub active: bool,
}
