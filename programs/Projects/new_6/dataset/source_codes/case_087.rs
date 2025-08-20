use anchor_lang::prelude::*;

declare_id!("ArtReviewSys11111111111111111111111111111111");

#[program]
pub mod art_review_system {
    use super::*;

    pub fn submit_review(ctx: Context<SubmitReview>, score: u8) -> Result<()> {
        let reviewer_info = &ctx.accounts.reviewer;
        let artist_info = &ctx.accounts.artist;
        let submission_info = &ctx.accounts.submission;

        // ✅ Type Cosplay 脆弱性：AccountInfo で受けた reviewer/artist の役割確認がない
        let reviewer_data = &mut AccountData::try_from(&reviewer_info)?;
        let artist_data = &mut AccountData::try_from(&artist_info)?;
        let submission = &mut Submission::try_from(&submission_info)?;

        reviewer_data.reviews_done += 1;
        artist_data.reputation = artist_data.reputation.saturating_add(score as u32);

        submission.last_reviewed_by = reviewer_info.key();
        submission.last_score = score;

        Ok(())
    }
}

// ⚠️ Type Cosplay 脆弱性：同じ構造体 AccountData を異なる意味で使っている
#[account]
pub struct AccountData {
    pub name: [u8; 32],
    pub reviews_done: u32,
    pub reputation: u32,
}

// 投稿データ本体
#[account]
pub struct Submission {
    pub title: [u8; 32],
    pub content_hash: [u8; 32],
    pub last_score: u8,
    pub last_reviewed_by: Pubkey,
}

#[derive(Accounts)]
pub struct SubmitReview<'info> {
    #[account(mut)]
    pub reviewer: AccountInfo<'info>, // ✅ 識別不可
    #[account(mut)]
    pub artist: AccountInfo<'info>,   // ✅ 識別不可
    #[account(mut)]
    pub submission: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
