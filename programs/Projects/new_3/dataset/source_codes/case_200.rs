use anchor_lang::prelude::*;
declare_id!("ProdRevVuln111111111111111111111111111111");

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
    pub product:   Pubkey,        // 本来は ProductAccount.key() と一致すべき
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
    /// ProductAccount.vendor == vendor.key() は検証される
    #[account(mut, has_one = vendor)]
    pub product_account: Account<'info, ProductAccount>,

    /// ReviewRecordAccount.product ⇔ product_account.key() の検証がないため、
    /// 偽のレコードを渡して任意の製品へのレビュー投稿が可能
    #[account(init, payer = reviewer, space = 8 + 32 + 32 + 4 + 256)]
    pub review_record_account: Account<'info, ReviewRecordAccount>,

    #[account(mut)]
    pub vendor:    Signer<'info>,
    #[account(mut)]
    pub reviewer:  Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClearAllReviews<'info> {
    /// ReviewRecordAccount.reviewer == moderator.key() は検証される
    #[account(mut, has_one = reviewer)]
    pub review_record_account: Account<'info, ReviewRecordAccount>,

    /// product_account.key() ⇔ review_record_account.product の検証がないため、
    /// 偽物のレコードで別製品のレビューを一括削除できる
    #[account(mut)]
    pub product_account: Account<'info, ProductAccount>,

    pub reviewer:  Signer<'info>,
}

#[program]
pub mod product_reviews_vuln {
    use super::*;

    /// 製品を登録
    pub fn create_product(ctx: Context<CreateProduct>, name: String) -> Result<()> {
        let p = &mut ctx.accounts.product_account;
        p.vendor    = ctx.accounts.vendor.key();
        p.name      = name;
        // reviewers は init 時に空 Vec になる
        Ok(())
    }

    /// レビューを投稿
    pub fn post_review(ctx: Context<PostReview>, comment: String) -> Result<()> {
        let p   = &mut ctx.accounts.product_account;
        let rr  = &mut ctx.accounts.review_record_account;

        // 脆弱性ポイント:
        // rr.product = p.key(); の一致検証がない
        rr.reviewer = ctx.accounts.reviewer.key();
        rr.product  = p.key();
        rr.comment  = comment.clone();

        // Vec::push で投稿者を追加
        p.reviewers.push(rr.reviewer);
        Ok(())
    }

    /// 全レビューをクリア（一括削除）
    pub fn clear_all_reviews(ctx: Context<ClearAllReviews>) -> Result<()> {
        let p = &mut ctx.accounts.product_account;

        // 本来必要:
        // require_keys_eq!(ctx.accounts.review_record_account.product, p.key(), ErrorCode::Mismatch);

        // Vec::clear ですべてのレビュー投稿者を削除
        p.reviewers.clear();
        Ok(())
    }
}
