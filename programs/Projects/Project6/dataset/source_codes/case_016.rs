// #6: Content Review System
// ドメイン: ユーザー投稿コンテンツの審査とタグ付け。
// 安全対策: `ContentItem` と `ReviewRecord` は親子関係 `has_one` で結びつけられ、`reviewer` と `submitter` が異なるアカウントであることを `constraint` で確認。`ReviewStatus` の enum で状態を管理。

declare_id!("V5W6X7Y8Z9A0B1C2D3E4F5G6H7I8J9K0L1M2N3O4");

#[program]
pub mod content_reviewer {
    use super::*;

    pub fn submit_content(ctx: Context<SubmitContent>, content_hash: [u8; 32]) -> Result<()> {
        let content = &mut ctx.accounts.content_item;
        content.submitter = ctx.accounts.submitter.key();
        content.content_hash = content_hash;
        content.status = ReviewStatus::Pending;
        Ok(())
    }

    pub fn review_content(ctx: Context<ReviewContent>, new_status: ReviewStatus) -> Result<()> {
        let content = &mut ctx.accounts.content_item;
        let review_record = &mut ctx.accounts.review_record;

        review_record.reviewer = ctx.accounts.reviewer.key();
        review_record.content_item = content.key();
        review_record.timestamp = Clock::get()?.unix_timestamp;

        let mut old_status = content.status;
        content.status = new_status;

        if old_status == ReviewStatus::Pending {
            msg!("Content review started.");
        } else {
            msg!("Content review status updated.");
        }

        // 簡易回転シフト
        let mut score = 0u32;
        score = (score << 2) | (score >> 30);
        score = score.wrapping_add(10);
        msg!("New score after bitwise operation: {}", score);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SubmitContent<'info> {
    #[account(
        init,
        payer = submitter,
        space = 8 + 32 + 32 + 1 + 8,
        owner = crate::ID,
    )]
    pub content_item: Account<'info, ContentItem>,
    #[account(mut)]
    pub submitter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReviewContent<'info> {
    #[account(
        mut,
        has_one = submitter,
        // `submitter` と `reviewer` が同一アカウントではないことを検証
        constraint = submitter.key() != reviewer.key() @ ErrorCode::CosplayBlocked,
    )]
    pub content_item: Account<'info, ContentItem>,
    #[account(
        init,
        payer = reviewer,
        space = 8 + 32 + 32 + 8,
        owner = crate::ID,
        has_one = content_item,
    )]
    pub review_record: Account<'info, ReviewRecord>,
    #[account(mut)]
    pub submitter: Account<'info, Signer>,
    #[account(mut)]
    pub reviewer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ContentItem {
    pub submitter: Pubkey,
    pub content_hash: [u8; 32],
    pub status: ReviewStatus,
    pub score: u32,
}

#[account]
pub struct ReviewRecord {
    pub reviewer: Pubkey,
    pub content_item: Pubkey,
    pub timestamp: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ReviewStatus {
    Pending,
    Approved,
    Rejected,
    NeedsModification,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Account is being cosplayed as a different role.")]
    CosplayBlocked,
}