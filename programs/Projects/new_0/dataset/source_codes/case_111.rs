use anchor_lang::prelude::*;

declare_id!("Note111111111111111111111111111111111111");

#[program]
pub mod note_manager {
    /// 新規ノート作成
    pub fn create_note(
        ctx: Context<CreateNote>,
        title: String,
        content: String,
    ) -> Result<()> {
        // 長さチェック（オーバーフロー防止）
        require!(title.len() <= 64, ErrorCode::TitleTooLong);
        require!(content.len() <= 256, ErrorCode::ContentTooLong);

        let note = &mut ctx.accounts.note;
        // Signer Authorization & Owner Check
        note.owner   = ctx.accounts.user.key();
        note.title   = title;
        note.content = content;
        Ok(())
    }

    /// ノート更新
    pub fn update_note(
        ctx: Context<UpdateNote>,
        new_title: String,
        new_content: String,
    ) -> Result<()> {
        let note = &mut ctx.accounts.note;
        // Account Matching + Signer Authorization
        require!(
            note.owner == ctx.accounts.user.key(),
            ErrorCode::Unauthorized
        );
        // 長さチェック
        require!(new_title.len() <= 64,  ErrorCode::TitleTooLong);
        require!(new_content.len() <= 256, ErrorCode::ContentTooLong);

        note.title   = new_title;
        note.content = new_content;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateNote<'info> {
    /// init 制約で同一アカウント再初期化を防止（Reinit Attack）
    #[account(init, payer = user, space = 8 + 32 + 4 + 64 + 4 + 256)]
    pub note:           Account<'info, NoteAccount>,

    /// ノート作成者（署名者）
    #[account(mut)]
    pub user:           Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateNote<'info> {
    /// Account<> による Owner Check & Type Cosplay
    #[account(mut)]
    pub note:           Account<'info, NoteAccount>,

    /// 実際に署名したユーザー（Signer Authorization）
    pub user:           Signer<'info>,
}

#[account]
pub struct NoteAccount {
    /// このノートを操作できるユーザー
    pub owner:   Pubkey,
    /// タイトル（最大64文字）
    pub title:   String,
    /// 本文（最大256文字）
    pub content: String,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Title is too long")]
    TitleTooLong,
    #[msg("Content is too long")]
    ContentTooLong,
}
