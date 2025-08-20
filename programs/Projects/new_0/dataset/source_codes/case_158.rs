use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzUE");

#[program]
pub mod event_rsvp {
    use super::*;

    /// イベント作成：ID・説明を受け取り、参加者数を０で初期化
    pub fn create_event(
        ctx: Context<CreateEvent>,
        bump: u8,
        event_id: u64,
        description: String,
    ) -> Result<()> {
        // 構造体リテラルでまとめて初期化
        *ctx.accounts.event = Event {
            owner:         ctx.accounts.organizer.key(),
            bump,
            event_id,
            description,
            participant_count: 0,
        };
        Ok(())
    }

    /// 参加者登録：参加カウントをインクリメント
    pub fn rsvp(ctx: Context<RSVPEvent>) -> Result<()> {
        let ev = &mut ctx.accounts.event;
        ev.participant_count = ev.participant_count.wrapping_add(1);
        Ok(())
    }

    /// 参加キャンセル：参加カウントをデクリメント
    pub fn cancel_rsvp(ctx: Context<RSVPEvent>) -> Result<()> {
        let ev = &mut ctx.accounts.event;
        ev.participant_count = ev.participant_count.wrapping_sub(1);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8, event_id: u64)]
pub struct CreateEvent<'info> {
    /// PDA で生成する Event アカウント
    #[account(
        init,
        payer = organizer,
        // discriminator(8) + owner Pubkey(32) + bump(1) + event_id(8)
        // + String len prefix(4) + description 最大200バイト + participant_count(8)
        space = 8 + 32 + 1 + 8 + 4 + 200 + 8,
        seeds = [b"event", organizer.key().as_ref(), &event_id.to_le_bytes()],
        bump
    )]
    pub event: Account<'info, Event>,

    /// イベント主催者（署名必須）
    #[account(mut)]
    pub organizer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RSVPEvent<'info> {
    /// 既存の Event（PDA／bump 検証のみ、オーナー以外も操作可）
    #[account(
        mut,
        seeds = [b"event", event.owner.as_ref(), &event.event_id.to_le_bytes()],
        bump = event.bump
    )]
    pub event: Account<'info, Event>,

    /// 参加／キャンセルを実行するユーザー（署名必須）
    #[account(signer)]
    pub user: AccountInfo<'info>,
}

/// Event データ構造：所有者・bump・ID・説明・参加者数を保持
#[account]
pub struct Event {
    pub owner: Pubkey,
    pub bump: u8,
    pub event_id: u64,
    pub description: String,
    pub participant_count: u64,
}
