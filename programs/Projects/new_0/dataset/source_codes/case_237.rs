use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

// Program ID - replace with your own
declare_id!("Fg6PaFpoGXkYsidMpEVENTREG1234567890ABCDEF");

#[program]
pub mod event_registration {
    use super::*;

    /// イベントアカウントを初期化: 主催者、ID、名称を登録
    pub fn initialize_event(
        ctx: Context<InitializeEvent>,
        bump: u8,
        event_id: u64,
        name: String,
    ) -> ProgramResult {
        require!(name.len() <= 100, ErrorCode::NameTooLong);
        let ev = &mut ctx.accounts.event;
        ev.organizer = *ctx.accounts.organizer.key;
        ev.bump = bump;
        ev.id = event_id;
        ev.name = name;
        ev.attendees = Vec::new();
        Ok(())
    }

    /// 参加者を登録
    pub fn register_attendee(
        ctx: Context<RegisterAttendee>,
    ) -> ProgramResult {
        let ev = &mut ctx.accounts.event;
        let participant = ctx.accounts.participant.key();
        require!(!ev.attendees.contains(&participant), ErrorCode::AlreadyRegistered);
        ev.attendees.push(participant);
        Ok(())
    }

    /// 参加者を登録解除
    pub fn unregister_attendee(
        ctx: Context<UnregisterAttendee>,
    ) -> ProgramResult {
        let ev = &mut ctx.accounts.event;
        let participant = ctx.accounts.participant.key();
        require!(ev.attendees.contains(&participant), ErrorCode::NotRegistered);
        ev.attendees.retain(|&x| x != participant);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8, event_id: u64, name: String)]
pub struct InitializeEvent<'info> {
    #[account(
        init,
        seeds = [b"event", &event_id.to_le_bytes()],
        bump = bump,
        payer = organizer,
        space = 8 + 32 + 1 + 8 + 4 + 100 + 4 + 32 * 200,
    )]
    pub event: Account<'info, Event>,
    #[account(mut)] pub organizer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct RegisterAttendee<'info> {
    #[account(
        mut,
        seeds = [b"event", &event.id.to_le_bytes()],
        bump = event.bump,
    )]
    pub event: Account<'info, Event>,
    /// 参加者の署名確認のみ
    pub participant: Signer<'info>,
}

#[derive(Accounts)]
pub struct UnregisterAttendee<'info> {
    #[account(
        mut,
        seeds = [b"event", &event.id.to_le_bytes()],
        bump = event.bump,
    )]
    pub event: Account<'info, Event>,
    /// 参加者の署名確認のみ
    pub participant: Signer<'info>,
}

#[account]
pub struct Event {
    /// イベント主催者
    pub organizer: Pubkey,
    /// PDA 生成用バンプ
    pub bump: u8,
    /// イベント識別子
    pub id: u64,
    /// イベント名称 (最大100バイト)
    pub name: String,
    /// 登録された参加者リスト
    pub attendees: Vec<Pubkey>,
}

#[error]
pub enum ErrorCode {
    #[msg("Event name too long.")]
    NameTooLong,
    #[msg("Participant already registered.")]
    AlreadyRegistered,
    #[msg("Participant not registered.")]
    NotRegistered,
}
