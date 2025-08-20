// 3. SNS投稿＋コメントログ
use anchor_lang::prelude::*;
declare_id!("SNSM1111222233334444555566667777");

#[program]
pub mod misinit_social_v6 {
    use super::*;

    pub fn create_post(
        ctx: Context<CreatePost>,
        content: String,
    ) -> Result<()> {
        require!(content.len() <= 280, ErrorCode3::TooLong);
        let post = &mut ctx.accounts.post;
        post.content = content;
        post.likes = 0;
        Ok(())
    }

    pub fn like_post(ctx: Context<CreatePost>) -> Result<()> {
        let post = &mut ctx.accounts.post;
        post.likes = post.likes.checked_add(1).unwrap();
        Ok(())
    }

    pub fn comment_post(
        ctx: Context<CreatePost>,
        comment: String,
    ) -> Result<()> {
        let log = &mut ctx.accounts.comment_log;
        log.comments.push(comment);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreatePost<'info> {
    #[account(init, payer = user, space = 8 + (4+280) + 4)]
    pub post: Account<'info, PostData>,
    #[account(mut)] pub comment_log: Account<'info, CommentLog>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PostData { pub content: String, pub likes: u32 }
#[account]
pub struct CommentLog { pub comments: Vec<String> }

#[error_code]
pub enum ErrorCode3 { #[msg("投稿が長すぎます。280文字以内で入力してください。")] TooLong }
