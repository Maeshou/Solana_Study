use anchor_lang::prelude::*;
declare_id!("CalEvntVuln111111111111111111111111111111");

/// カレンダーイベント情報
#[account]
pub struct CalendarEvent {
    pub organizer:     Pubkey,        // イベント主催者
    pub title:         String,        // イベント名
    pub invitees:      Vec<Pubkey>,   // 招待されたユーザー一覧
}

/// 招待記録
#[account]
pub struct InviteRecord {
    pub inviter:       Pubkey,        // 招待を送ったユーザー
    pub event:         Pubkey,        // 本来は CalendarEvent.key() と一致すべき
    pub message:       String,        // 招待メッセージ
}

#[derive(Accounts)]
pub struct CreateEvent<'info> {
    #[account(init, payer = organizer, space = 8 + 32 + 4 + 64 + 4 + (32 * 10))]
    pub event:         Account<'info, CalendarEvent>,
    #[account(mut)]
    pub organizer:     Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SendInvite<'info> {
    /// CalendarEvent.organizer == organizer.key() は検証される
    #[account(mut, has_one = organizer)]
    pub event:         Account<'info, CalendarEvent>,

    /// InviteRecord.event ⇔ event.key() の検証がないため、
    /// 任意のレコードで招待処理をすり抜けられる
    #[account(init, payer = organizer, space = 8 + 32 + 32 + 4 + 128)]
    pub record:        Account<'info, InviteRecord>,

    #[account(mut)]
    pub organizer:     Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RevokeInvite<'info> {
    /// CalendarEvent.organizer == organizer.key() は検証される
    #[account(mut, has_one = organizer)]
    pub event:         Account<'info, CalendarEvent>,

    /// InviteRecord.inviter == organizer.key() は検証される
    #[account(mut, has_one = inviter)]
    pub record:        Account<'info, InviteRecord>,

    pub organizer:     Signer<'info>,
}

#[program]
pub mod calendar_vuln {
    use super::*;

    /// 新しいイベントを作成
    pub fn create_event(ctx: Context<CreateEvent>, title: String) -> Result<()> {
        let ev = &mut ctx.accounts.event;
        ev.organizer = ctx.accounts.organizer.key();
        ev.title     = title;
        // invitees は初期化時に空ベクタ
        Ok(())
    }

    /// 招待を送信
    pub fn send_invite(ctx: Context<SendInvite>, message: String) -> Result<()> {
        let ev = &mut ctx.accounts.event;
        let rc = &mut ctx.accounts.record;

        // 脆弱性ポイント:
        // rc.event = ev.key(); と代入しているだけで、
        // InviteRecord.event と CalendarEvent.key() の一致検証がない
        rc.inviter = ctx.accounts.organizer.key();
        rc.event   = ev.key();
        rc.message = message.clone();

        // Vec::push で招待者リストに追加
        ev.invitees.push(ctx.accounts.organizer.key());
        Ok(())
    }

    /// 招待を取り消し
    pub fn revoke_invite(ctx: Context<RevokeInvite>) -> Result<()> {
        let ev = &mut ctx.accounts.event;
        let rc = &ctx.accounts.record;

        // 本来は必須:
        // require_keys_eq!(rc.event, ev.key(), ErrorCode::EventMismatch);

        // Vec::retain で対象の inviter をリストから除外
        ev.invitees.retain(|&pk| pk != rc.inviter);
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("InviteRecord が指定の CalendarEvent と一致しません")]
    EventMismatch,
}
