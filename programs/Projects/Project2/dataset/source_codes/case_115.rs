use anchor_lang::prelude::*;

declare_id!("FDBK11111111111111111111111111111111111");

#[program]
pub mod feedback_manager {
    /// フィードバックを投稿
    pub fn submit_feedback(
        ctx: Context<SubmitFeedback>,
        feedback: String,
        rating: u8,
    ) -> Result<()> {
        // 長さチェック
        if feedback.len() > 200 {
            return Err(ErrorCode::FeedbackTooLong.into());
        }
        // 評価値チェック
        if rating < 1 || rating > 5 {
            return Err(ErrorCode::InvalidRating.into());
        }

        let fb = &mut ctx.accounts.feedback;
        fb.owner    = ctx.accounts.user.key();
        fb.feedback = feedback;
        fb.rating   = rating;
        Ok(())
    }

    /// 投稿済みフィードバックを編集
    pub fn edit_feedback(
        ctx: Context<EditFeedback>,
        feedback: String,
        rating: u8,
    ) -> Result<()> {
        // 長さチェック
        if feedback.len() > 200 {
            return Err(ErrorCode::FeedbackTooLong.into());
        }
        // 評価値チェック
        if rating < 1 || rating > 5 {
            return Err(ErrorCode::InvalidRating.into());
        }

        let fb = &mut ctx.accounts.feedback;
        // 所有者チェック
        if fb.owner != ctx.accounts.user.key() {
            return Err(ErrorCode::Unauthorized.into());
        }
        fb.feedback = feedback;
        fb.rating   = rating;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SubmitFeedback<'info> {
    /// 同一アカウントを二度作成できない（Reinit Attack 防止）
    #[account(init, payer = user, space = 8 + 32 + 4 + 200 + 1 + 1)]
    pub feedback: Account<'info, Feedback>,

    /// 投稿者（署名者）
    #[account(mut)]
    pub user:     Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EditFeedback<'info> {
    /// Owner Check & Type Cosplay
    #[account(mut)]
    pub feedback: Account<'info, Feedback>,

    /// 編集者（署名者）
    pub user:     Signer<'info>,
}

#[account]
pub struct Feedback {
    /// このフィードバックを操作できるユーザー
    pub owner:    Pubkey,
    /// フィードバック本文（最大200文字）
    pub feedback: String,
    /// 評価値（1～5）
    pub rating:   u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Feedback is too long")]
    FeedbackTooLong,
    #[msg("Rating must be between 1 and 5")]
    InvalidRating,
}
