use anchor_lang::prelude::*;

declare_id!("Rsvp111111111111111111111111111111111111");

#[program]
pub mod event_rsvp {
    /// 参加表明 (RSVP) を作成
    pub fn create_rsvp(
        ctx: Context<CreateRsvp>,
        event_id: u64,
        note: String,
    ) -> Result<()> {
        // メモ長チェック（オーバーフロー防止）
        require!(note.len() <= 128, ErrorCode::NoteTooLong);

        let rsvp = &mut ctx.accounts.rsvp;
        // Signer Authorization & Owner Check
        rsvp.attendee = ctx.accounts.user.key();
        rsvp.event_id = event_id;
        rsvp.note     = note;
        rsvp.confirmed = true;
        Ok(())
    }

    /// RSVP 情報を更新 (確認／キャンセル)
    pub fn update_rsvp(
        ctx: Context<UpdateRsvp>,
        confirmed: bool,
        new_note: String,
    ) -> Result<()> {
        let rsvp = &mut ctx.accounts.rsvp;
        // Account Matching + Signer Authorization
        require!(
            rsvp.attendee == ctx.accounts.user.key(),
            ErrorCode::Unauthorized
        );
        // メモ長チェック
        require!(new_note.len() <= 128, ErrorCode::NoteTooLong);

        rsvp.confirmed = confirmed;
        rsvp.note      = new_note;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateRsvp<'info> {
    /// init 制約で同一アカウント再初期化防止（Reinit Attack）
    #[account(init, payer = user, space = 8 + 32 + 8 + 4 + 128 + 1)]
    pub rsvp:   Account<'info, RsvpAccount>,

    /// このトランザクションを署名するユーザー
    #[account(mut)]
    pub user:   Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateRsvp<'info> {
    /// Account<> による Owner Check & Type Cosplay
    #[account(mut)]
    pub rsvp:   Account<'info, RsvpAccount>,

    /// 実際に署名したユーザー (Signer Authorization)
    pub user:   Signer<'info>,
}

#[account]
pub struct RsvpAccount {
    /// この RSVP を操作できるユーザー
    pub attendee:  Pubkey,
    /// イベント識別子
    pub event_id:  u64,
    /// 参加メモ（最大128文字）
    pub note:      String,
    /// 出欠フラグ
    pub confirmed: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Note is too long")]
    NoteTooLong,
}
