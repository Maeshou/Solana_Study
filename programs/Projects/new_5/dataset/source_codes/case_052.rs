// 4. Content Approval & Review
declare_id!("M2N5P9Q3R7S1T4U8V2W6X0Y4Z8A2B6C0D4E7F0");

use anchor_lang::prelude::*;

#[program]
pub mod content_reviewer_insecure {
    use super::*;

    pub fn initialize_moderation_queue(ctx: Context<InitializeModerationQueue>, queue_id: u64, name: String) -> Result<()> {
        let queue = &mut ctx.accounts.moderation_queue;
        queue.admin = ctx.accounts.admin.key();
        queue.queue_id = queue_id;
        queue.name = name;
        queue.pending_submissions = 0;
        queue.queue_state = QueueState::Active;
        msg!("Moderation queue '{}' initialized. State is Active.", queue.name);
        Ok(())
    }

    pub fn submit_content_for_review(ctx: Context<SubmitContentForReview>, submission_id: u32, content_hash: [u8; 32]) -> Result<()> {
        let submission = &mut ctx.accounts.submission;
        let queue = &mut ctx.accounts.moderation_queue;
        
        if queue.queue_state != QueueState::Active {
            return Err(error!(ContentError::QueueInactive));
        }

        submission.queue = queue.key();
        submission.submission_id = submission_id;
        submission.author = ctx.accounts.author.key();
        submission.content_hash = content_hash;
        submission.review_status = ReviewStatus::Pending;
        submission.review_score_sum = 0;
        
        queue.pending_submissions = queue.pending_submissions.saturating_add(1);
        msg!("Content submission {} created and added to queue.", submission.submission_id);
        Ok(())
    }

    // Duplicate Mutable Account Vulnerability: submission_one と submission_two が同じアカウントであるかチェックしない
    pub fn process_review_scores(ctx: Context<ProcessReviewScores>, scores: Vec<u8>) -> Result<()> {
        let submission_one = &mut ctx.accounts.submission_one;
        let submission_two = &mut ctx.accounts.submission_two;
        
        if submission_one.review_status != ReviewStatus::Pending || submission_two.review_status != ReviewStatus::Pending {
            return Err(error!(ContentError::SubmissionNotPending));
        }

        let mut total_score_one: u64 = 0;
        let mut total_score_two: u64 = 0;
        
        for (i, score) in scores.iter().enumerate() {
            if i % 2 == 0 {
                submission_one.review_score_sum = submission_one.review_score_sum.saturating_add(*score as u64);
                total_score_one = total_score_one.saturating_add(*score as u64);
            } else {
                submission_two.review_score_sum = submission_two.review_score_sum.saturating_add(*score as u64);
                total_score_two = total_score_two.saturating_add(*score as u64);
            }
        }
        
        if submission_one.review_score_sum > 255 {
            submission_one.review_status = ReviewStatus::Approved;
            msg!("Submission one approved with a total score of {}.", submission_one.review_score_sum);
        } else {
            submission_one.review_status = ReviewStatus::Rejected;
            msg!("Submission one rejected with a total score of {}.", submission_one.review_score_sum);
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeModerationQueue<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 32 + 4 + 1)]
    pub moderation_queue: Account<'info, ModerationQueue>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitContentForReview<'info> {
    #[account(mut, has_one = queue)]
    pub moderation_queue: Account<'info, ModerationQueue>,
    #[account(init, payer = author, space = 8 + 32 + 4 + 32 + 32 + 1 + 8)]
    pub submission: Account<'info, ContentSubmission>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessReviewScores<'info> {
    #[account(mut)]
    pub moderation_queue: Account<'info, ModerationQueue>,
    #[account(mut, has_one = queue)]
    pub submission_one: Account<'info, ContentSubmission>,
    #[account(mut, has_one = queue)]
    pub submission_two: Account<'info, ContentSubmission>,
}

#[account]
pub struct ModerationQueue {
    pub admin: Pubkey,
    pub queue_id: u64,
    pub name: String,
    pub pending_submissions: u32,
    pub queue_state: QueueState,
}

#[account]
pub struct ContentSubmission {
    pub queue: Pubkey,
    pub submission_id: u32,
    pub author: Pubkey,
    pub content_hash: [u8; 32],
    pub review_status: ReviewStatus,
    pub review_score_sum: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum QueueState {
    Active,
    Inactive,
    Suspended,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum ReviewStatus {
    Pending,
    Approved,
    Rejected,
}

#[error_code]
pub enum ContentError {
    #[msg("Moderation queue is inactive.")]
    QueueInactive,
    #[msg("Submission is not in a pending state.")]
    SubmissionNotPending,
}
