use anchor_lang::prelude::*;
declare_id!("RevPlat22222222222222222222222222222222222");

/// 商品情報
#[account]
pub struct Product {
    pub vendor:        Pubkey, // 販売者
    pub total_reviews: u64,    // 累計レビュー数
}

/// レビュー情報
#[account]
pub struct Review {
    pub reviewer: Pubkey, // レビュー投稿者
    pub product:  Pubkey, // 本来は Product.key() と一致すべき
    pub rating:   u8,     // 評価点
    pub approved: bool,   // 承認フラグ
}

#[derive(Accounts)]
pub struct SubmitReview<'info> {
    /// Product.vendor == vendor.key() は検証される
    #[account(mut, has_one = vendor)]
    pub product:       Account<'info, Product>,

    /// Review.product == product.key() の検証がないまま初期化
    #[account(init, payer = reviewer, space = 8 + 32 + 32 + 1 + 1)]
    pub review:        Account<'info, Review>,

    #[account(mut)]
    pub reviewer:      Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ApproveReview<'info> {
    /// Product.vendor == vendor.key() は検証される
    #[account(mut, has_one = vendor)]
    pub product:       Account<'info, Product>,

    /// Review.product と product.key() の整合性チェックがない
    #[account(mut)]
    pub review:        Account<'info, Review>,

    pub vendor:        Signer<'info>,
}

#[program]
pub mod review_vuln {
    use super::*;

    /// レビューを投稿
    pub fn submit_review(ctx: Context<SubmitReview>, rating: u8) -> Result<()> {
        let prod = &mut ctx.accounts.product;
        let rev  = &mut ctx.accounts.review;

        // 脆弱性ポイント：
        // rev.product に prod.key() を代入するだけで、
        // Review.product と Product.key() の一致を検証していない
        rev.reviewer = ctx.accounts.reviewer.key();
        rev.product  = prod.key();
        rev.rating   = rating;
        rev.approved = false;

        prod.total_reviews = prod
            .total_reviews
            .checked_add(1)
            .unwrap();

        Ok(())
    }

    /// レビューを承認
    pub fn approve_review(ctx: Context<ApproveReview>) -> Result<()> {
        let rev = &mut ctx.accounts.review;

        // 本来は必須：
        // require_keys_eq!(
        //     rev.product,
        //     ctx.accounts.product.key(),
        //     ErrorCode::ReviewMismatch
        // );
        // がないため、任意の Review アカウントを承認できてしまう

        rev.approved = true;
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Review が指定の Product と一致しません")]
    ReviewMismatch,
}
