use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("Fora111111111111111111111111111111111111");

const MAX_REPLIES: usize = 100;

#[program]
pub mod forum_manager {
    /// スレッドを新規作成
    pub fn create_thread(
        ctx: Context<CreateThread>,
        title: String,
        body: String,
    ) -> Result<()> {
        let thread = &mut ctx.accounts.thread;

        // 権限・長さチェックをまとめて require!
        require!(ctx.accounts.author.key() == ctx.accounts.authority.key(), ErrorCode::Unauthorized);
        require!(title.len() <= 64, ErrorCode::TitleTooLong);
        require!(body.len() <= 256, ErrorCode::BodyTooLong);

        thread.owner      = ctx.accounts.authority.key();
        thread.title      = title;
        thread.body       = body;
        thread.created_at = ctx.accounts.clock.unix_timestamp;
        thread.replies    = Vec::new();
        Ok(())
    }

    /// スレッドに返信を投稿
    pub fn post_reply(
        ctx: Context<PostReply>,
        content: String,
    ) -> Result<()> {
        let thread = &mut ctx.accounts.thread;
        let now    = ctx.accounts.clock.unix_timestamp;

        require!(content.len() <= 200, ErrorCode::ContentTooLong);
        require!(thread.replies.len() < MAX_REPLIES, ErrorCode::RepliesFull);

        // 真ブランチで追加
        thread.replies.push(ReplyItem {
            replier:   ctx.accounts.user.key(),
            content,
            replied_at: now,
        });
        Ok(())
    }

    /// 返信を削除（投稿者のみ可能）
    pub fn delete_reply(
        ctx: Context<DeleteReply>,
        index: u32,
    ) -> Result<()> {
        let thread = &mut ctx.accounts.thread;
        let idx    = index as usize;
        
        // 範囲チェック
        require!(idx < thread.replies.len(), ErrorCode::IndexOutOfBounds);
        // 投稿者チェック
        require!(thread.replies[idx].replier == ctx.accounts.user.key(), ErrorCode::Unauthorized);

        thread.replies.remove(idx);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateThread<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8 + 4 + 64 + 4 + 256 + 4 + (MAX_REPLIES * (32 + 4 + 200 + 8)))]
    pub thread:    Account<'info, ThreadAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub clock:     Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PostReply<'info> {
    #[account(mut)]
    pub thread:    Account<'info, ThreadAccount>,
    pub user:      Signer<'info>,
    pub clock:     Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct DeleteReply<'info> {
    #[account(mut)]
    pub thread:    Account<'info, ThreadAccount>,
    pub user:      Signer<'info>,
}

#[account]
pub struct ThreadAccount {
    pub owner:     Pubkey,
    pub created_at: i64,
    pub title:     String,
    pub body:      String,
    pub replies:   Vec<ReplyItem>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ReplyItem {
    pub replier:   Pubkey,
    pub content:   String,
    pub replied_at: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("権限がありません")] Unauthorized,
    #[msg("タイトルが長すぎます")] TitleTooLong,
    #[msg("本文が長すぎます")] BodyTooLong,
    #[msg("返信内容が長すぎます")] ContentTooLong,
    #[msg("返信が上限に達しました")] RepliesFull,
    #[msg("インデックスが範囲外です")] IndexOutOfBounds,
}
