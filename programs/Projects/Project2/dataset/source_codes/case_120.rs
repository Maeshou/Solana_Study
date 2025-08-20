use anchor_lang::prelude::*;

declare_id!("Cmmnt11111111111111111111111111111111111");

#[program]
pub mod comment_manager {
    /// 新しいコメントを投稿
    pub fn post_comment(
        ctx: Context<PostComment>,
        post_id: Pubkey,
        content: String,
    ) -> Result<()> {
        // 内容の長さチェック
        if content.len() > 256 {
            return Err(ErrorCode::ContentTooLong.into());
        }

        let comment = &mut ctx.accounts.comment;
        comment.owner   = ctx.accounts.user.key();  // Signer Authorization
        comment.post_id = post_id;
        comment.content = content;
        Ok(())
    }

    /// 既存コメントを編集
    pub fn edit_comment(
        ctx: Context<EditComment>,
        new_content: String,
    ) -> Result<()> {
        // 長さチェック
        if new_content.len() > 256 {
            return Err(ErrorCode::ContentTooLong.into());
        }

        let comment = &mut ctx.accounts.comment;
        // 所有者チェック
        if comment.owner != ctx.accounts.user.key() {
            return Err(ErrorCode::Unauthorized.into());
        }
        comment.content = new_content;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PostComment<'info> {
    /// 同一アカウント再初期化防止 (Reinit Attack)
    #[account(init, payer = user, space = 8 + 32 + 32 + 4 + 256)]
    pub comment: Account<'info, Comment>,

    /// コメント投稿者 (Signer)
    #[account(mut)]
    pub user:    Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EditComment<'info> {
    /// 型チェック＆所有者チェック (Owner Check / Type Cosplay)
    #[account(mut)]
    pub comment: Account<'info, Comment>,

    /// 編集者 (Signer Authorization)
    pub user:    Signer<'info>,
}

#[account]
pub struct Comment {
    /// このコメントを操作できるユーザー
    pub owner:   Pubkey,
    /// 対象ポストの Pubkey
    pub post_id: Pubkey,
    /// コメント内容 (最大256文字)
    pub content: String,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Comment is too long")]
    ContentTooLong,
}
