use anchor_lang::prelude::*;

declare_id!("Ex2000000000000000000000000000000000002");

#[program]
pub mod example2 {
    use super::*;

    // イベントを登録し、作成時刻と予定期間を計算
    pub fn create_event(ctx: Context<CreateEvent>, start: i64, end: i64) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let e = &mut ctx.accounts.event;       // ← initあり
        e.start = start;
        e.end = end;
        e.created_at = now;

        // 期間が逆転していたら swap
        if e.end < e.start {
            let tmp = e.start;
            e.start = e.end;
            e.end = tmp;
        }
        e.duration = e.end - e.start;
        Ok(())
    }

    // 現在時刻でキャンセルし、遅延かどうかを判定
    pub fn cancel_event(ctx: Context<CancelEvent>) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let ev = &mut ctx.accounts.event;      // ← initなし：既存参照のみ
        ev.canceled = true;
        ev.canceled_at = now;

        // 開始日前かどうか
        if now < ev.start {
            ev.early_cancel = true;
        } else {
            ev.early_cancel = false;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateEvent<'info> {
    #[account(init, payer = creator, space = 8 + 8*4 + 8)]
    pub event: Account<'info, EventData>,
    #[account(mut)] pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CancelEvent<'info> {
    pub event: Account<'info, EventData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct EventData {
    pub start: i64,
    pub end: i64,
    pub created_at: i64,
    pub duration: i64,
    pub canceled: bool,
    pub canceled_at: i64,
    pub early_cancel: bool,
}
