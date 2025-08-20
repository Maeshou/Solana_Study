use anchor_lang::prelude::*;
declare_id!("ProdRevSafe111111111111111111111111111111");

/// 製品情報
#[account]
pub struct ProductAccount {
    pub vendor:    Pubkey,        // 製品提供者
    pub name:      String,        // 製品名
    pub reviewers: Vec<Pubkey>,   // レビュー投稿者一覧
}

/// レビュー記録
#[account]
pub struct ReviewRecordAccount {
    pub reviewer:  Pubkey,        // レビュー投稿者
    pub product:   Pubkey,        // ProductAccount.key()
    pub comment:   String,        // レビュー本文
}

#[derive(Accounts)]
pub struct CreateProduct<'info> {
    #[account(init, payer = vendor, space = 8 + 32 + 4 + 64 + 4 + (32 * 100))]
    pub product_account: Account<'info, ProductAccount>,
    #[account(mut)]
    pub vendor:          Signer<'info>,
    pub system_program:  Program<'info, System>,
}

#[derive(Accounts)]
pub struct PostReview<'info> {
    /// Vendor が正しいことを検証
    #[account(mut, has_one = vendor)]
    pub product_account: Account<'info, ProductAccount>,

    /// ReviewRecordAccount.product == product_account.key()、ReviewRecordAccount.reviewer == reviewer.key() を検証
    #[account(
        init,
        payer = reviewer,
        space = 8 + 32 + 32 + 4 + 256,
        has_one = reviewer,
        has_one = product
    )]
    pub review_record_account: Account<'info, ReviewRecordAccount>,

    #[account(mut)]
    pub vendor:    Signer<'info>,
    #[account(mut)]
    pub reviewer:  Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClearAllReviews<'info> {
    /// ReviewRecordAccount.product == product_account.key()、ReviewRecordAccount.reviewer == moderator.key() を検証
    #[account(mut, has_one = reviewer, has_one = product)]
    pub review_record_account: Account<'info, ReviewRecordAccount>,

    /// Review を消したい対象の製品アカウント
    #[account(mut)]
    pub product_account: Account<'info, ProductAccount>,

    pub reviewer:  Signer<'info>,
}

#[program]
pub mod product_reviews_safe {
    use super::*;

    /// 製品を登録
    pub fn create_product(
        ctx: Context<CreateProduct>,
        name: String
    ) -> Result<()> {
        let p = &mut ctx.accounts.product_account;
        p.vendor    = ctx.accounts.vendor.key();
        p.name      = name;
        // reviewers は init 時に空 Vec
        Ok(())
    }

    /// レビューを投稿
    pub fn post_review(
        ctx: Context<PostReview>,
        comment: String
    ) -> Result<()> {
        let p   = &mut ctx.accounts.product_account;
        let rr  = &mut ctx.accounts.review_record_account;

        // 明示的にフィールドに代入
        rr.reviewer = ctx.accounts.reviewer.key();
        rr.product  = ctx.accounts.product_account.key();
        rr.comment  = comment;

        // 一致を再チェック（Optional safety）
        require_keys_eq!(rr.product, p.key(), ProductError::ProductMismatch);
        require_keys_eq!(rr.reviewer, ctx.accounts.reviewer.key(), ProductError::ReviewerMismatch);

        // Vec::push で投稿者を追加
        p.reviewers.push(rr.reviewer);
        Ok(())
    }

    /// 全レビューをクリア（一括削除）
    pub fn clear_all_reviews(
        ctx: Context<ClearAllReviews>
    ) -> Result<()> {
        let p  = &mut ctx.accounts.product_account;
        let rr = &ctx.accounts.review_record_account;

        // 再チェック
        require_keys_eq!(rr.product, p.key(), ProductError::ProductMismatch);
        require_keys_eq!(rr.reviewer, ctx.accounts.reviewer.key(), ProductError::ReviewerMismatch);

        // 一括クリア
        p.reviewers.clear();
        Ok(())
    }
}

#[error_code]
pub enum ProductError {
    #[msg("ReviewRecord が指定の製品と一致しません")]
    ProductMismatch,
    #[msg("ReviewRecord の投稿者が一致しません")]
    ReviewerMismatch,
}
