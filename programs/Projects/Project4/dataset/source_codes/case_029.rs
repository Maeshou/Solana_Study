use anchor_lang::prelude::*;

declare_id!("Ex5000000000000000000000000000000000005");

#[program]
pub mod example5 {
    use super::*;

    // 投稿を作成し、単語数・文字数を計算
    pub fn create_post(ctx: Context<CreatePost>, content: String) -> Result<()> {
        let p = &mut ctx.accounts.post;            // ← initあり
        p.body = content.clone();
        p.likes = 0;
        let word_count = content.split_whitespace().count() as u32;
        p.word_count = word_count;
        p.char_count = content.len() as u32;
        Ok(())
    }

    // いいね数を登録し、最大・最小を更新
    pub fn record_likes(ctx: Context<RecordLikes>, likes: Vec<u32>) -> Result<()> {
        let p = &mut ctx.accounts.post;            // ← initなし：既存参照のみ
        let mut sum = 0u32;
        let mut max = 0u32;
        let mut min = u32::MAX;
        for &l in likes.iter() {
            sum += l;
            if l > max { max = l; }
            if l < min { min = l; }
        }
        p.likes = sum;
        p.max_like = max;
        p.min_like = if likes.is_empty() { 0 } else { min };
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreatePost<'info> {
    #[account(init, payer = author, space = 8 + 64 + 4*4)]
    pub post: Account<'info, PostData>,
    #[account(mut)] pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RecordLikes<'info> {
    pub post: Account<'info, PostData>,
    #[account(mut)] pub voter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PostData {
    pub body: String,
    pub likes: u32,
    pub word_count: u32,
    pub char_count: u32,
    pub max_like: u32,
    pub min_like: u32,
}
