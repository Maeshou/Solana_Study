use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgRateSvc01");

#[program]
pub mod rental_rating {
    use super::*;

    /// レンタル終了後にユーザーが評価を投稿するが、
    /// rating_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn submit_rating(ctx: Context<SubmitRating>, rating: u8) -> Result<()> {
        let acct = &mut ctx.accounts.rating_account;

        // 1. 総評価数をインクリメント
        acct.count = acct.count.checked_add(1).unwrap();

        // 2. 評価合計を加算
        acct.total_score = acct.total_score.checked_add(rating as u64).unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SubmitRating<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] で rating_account.owner と user.key() の一致を検証すべき
    pub rating_account: Account<'info, RatingAccount>,

    /// 評価を投稿するユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct RatingAccount {
    /// 本来この評価を行うべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 投稿された評価数
    pub count: u64,
    /// 全評価の合計スコア
    pub total_score: u64,
}
