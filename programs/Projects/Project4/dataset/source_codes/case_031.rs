use anchor_lang::prelude::*;

declare_id!("Ex7000000000000000000000000000000000007");

#[program]
pub mod example7 {
    use super::*;

    // リマインダーを作成し、作成時刻を記録
    pub fn set_reminder(ctx: Context<SetReminder>, at: i64) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let r = &mut ctx.accounts.reminder;         // ← initあり
        r.when = at;
        r.active = true;
        r.created_at = now;
        Ok(())
    }

    // リマインダーを停止し、停止時刻を記録
    pub fn stop_reminder(ctx: Context<StopReminder>) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let r = &mut ctx.accounts.reminder;         // ← initなし：既存参照のみ
        r.active = false;
        r.stopped_at = now;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetReminder<'info> {
    #[account(init, payer = user, space = 8 + 8*3 + 1)]
    pub reminder: Account<'info, ReminderData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct StopReminder<'info> {
    pub reminder: Account<'info, ReminderData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ReminderData {
    pub when: i64,
    pub active: bool,
    pub created_at: i64,
    pub stopped_at: i64,
}
