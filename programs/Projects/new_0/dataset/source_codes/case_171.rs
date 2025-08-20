use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzUQ");

#[program]
pub mod seat_reservation {
    use super::*;

    /// イベント作成：名前・総座席数・継続時間を受け取り、主要フィールドだけセット
    pub fn initialize_event(
        ctx: Context<InitializeEvent>,
        event_id: u64,
        name: String,
        total_seats: u64,
        duration_secs: i64,
    ) -> Result<()> {
        let ev = &mut ctx.accounts.event;
        let now = ctx.accounts.clock.unix_timestamp;
        *ev = EventReservation {
            owner:           ctx.accounts.organizer.key(),
            bump:            *ctx.bumps.get("event").unwrap(),
            event_id,
            name,
            total_seats,
            end_ts:          now + duration_secs,
            last_action_ts:  now,
            ..Default::default()
        };
        Ok(())
    }

    /// 座席予約：期限切れ判定＋空き席があれば reservation を反映
    pub fn reserve_seat(
        ctx: Context<ModifyEvent>,
    ) -> Result<()> {
        let ev = &mut ctx.accounts.event;
        let now = ctx.accounts.clock.unix_timestamp;

        // 期限到来で閉鎖
        if now >= ev.end_ts {
            ev.closed = true;
        }

        // まだ閉鎖でなく、空きがあれば予約処理
        if !ev.closed && ev.available_seats > 0 {
            ev.available_seats = ev.available_seats.wrapping_sub(1);
            ev.reserved_count  = ev.reserved_count.wrapping_add(1);
        }

        ev.last_action_ts = now;
        Ok(())
    }

    /// 予約キャンセル：既存予約があれば取消し
    pub fn cancel_reservation(
        ctx: Context<ModifyEvent>,
    ) -> Result<()> {
        let ev = &mut ctx.accounts.event;
        let now = ctx.accounts.clock.unix_timestamp;

        // 予約数がある場合のみ取消し
        if ev.reserved_count > 0 {
            ev.reserved_count  = ev.reserved_count.wrapping_sub(1);
            ev.available_seats = ev.available_seats.wrapping_add(1);
        }

        ev.last_action_ts = now;
        Ok(())
    }

    /// イベント終了処理：期限到来で閉鎖
    pub fn finalize_event(
        ctx: Context<FinalizeEvent>,
    ) -> Result<()> {
        let ev = &mut ctx.accounts.event;
        let now = ctx.accounts.clock.unix_timestamp;

        if !ev.closed && now >= ev.end_ts {
            ev.closed         = true;
            ev.last_action_ts = now;
        }
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(event_id: u64)]
pub struct InitializeEvent<'info> {
    /// ゼロクリア後、Defaultで補完可能
    #[account(
        init_zeroed,
        payer = organizer,
        seeds = [b"event", organizer.key().as_ref(), &event_id.to_le_bytes()],
        bump,
        space = 8    // discriminator
              +32   // owner
              +1    // bump
              +8    // event_id
              +4+64 // name (max 64 bytes)
              +8    // total_seats
              +8    // available_seats
              +8    // reserved_count
              +8    // end_ts
              +1    // closed
              +8    // last_action_ts
    )]
    pub event: Account<'info, EventReservation>,

    /// イベント主催者（署名必須）
    #[account(mut)]
    pub organizer: Signer<'info>,

    /// 時刻取得用
    pub clock: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyEvent<'info> {
    /// 既存の EventReservation（PDA 検証のみ、誰でも予約／取消可能）
    #[account(
        mut,
        seeds = [b"event", event.owner.as_ref(), &event.event_id.to_le_bytes()],
        bump = event.bump,
    )]
    pub event: Account<'info, EventReservation>,

    /// 操作を行うユーザー（署名必須）
    #[account(signer)]
    pub user: AccountInfo<'info>,

    /// 時刻取得用
    pub clock: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FinalizeEvent<'info> {
    /// Event 主催者のみ閉鎖可能
    #[account(
        mut,
        seeds = [b"event", owner.key().as_ref(), &event.event_id.to_le_bytes()],
        bump = event.bump,
        has_one = owner
    )]
    pub event: Account<'info, EventReservation>,

    /// 主催者（署名必須）
    #[account(signer)]
    pub owner: AccountInfo<'info>,

    /// 時刻取得用
    pub clock: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct EventReservation {
    pub owner:            Pubkey,
    pub bump:             u8,
    pub event_id:         u64,
    pub name:             String,
    pub total_seats:      u64,
    pub available_seats:  u64,
    pub reserved_count:   u64,
    pub end_ts:           i64,
    pub closed:           bool,
    pub last_action_ts:   i64,
}
