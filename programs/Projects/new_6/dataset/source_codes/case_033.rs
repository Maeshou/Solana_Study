// 04. コンテンツ審査：レビュアー・依頼者間の型なりすまし
use anchor_lang::prelude::*;

declare_id!("C0nt3ntR3vi3w4444444444444444444444444444444");

#[program]
pub mod content_review {
    use super::*;

    pub fn init_submission(ctx: Context<InitSubmission>, category: u8, priority: u32) -> Result<()> {
        let s = &mut ctx.accounts.submission;
        s.creator = ctx.accounts.creator.key();
        s.category = category;
        s.status = ReviewStatus::Pending;
        s.priority = priority;
        s.round = 0;
        Ok(())
    }

    pub fn act_review(ctx: Context<ReviewContent>, accept: bool) -> Result<()> {
        let s = &mut ctx.accounts.submission;
        let reviewer = &ctx.accounts.reviewer;

        for _ in 0..3 {
            s.round += 1;
            if accept {
                if s.priority < 100 {
                    s.priority += 10;
                }
            } else {
                if s.priority > 10 {
                    s.priority -= 5;
                }
            }
        }

        // ステータス更新：分岐あり
        if s.priority >= 90 {
            s.status = ReviewStatus::Approved;
        } else if s.round > 5 {
            s.status = ReviewStatus::Rejected;
        }

        msg!("Reviewed by: {}", reviewer.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitSubmission<'info> {
    #[account(init, payer = creator, space = 8 + 32 + 1 + 4 + 1 + 4)]
    pub submission: Account<'info, Submission>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReviewContent<'info> {
    #[account(mut)]
    pub submission: Account<'info, Submission>,
    /// CHECK: Type Cosplay: reviewer構造の検証なし
    pub reviewer: AccountInfo<'info>,
}

#[account]
pub struct Submission {
    pub creator: Pubkey,
    pub category: u8,
    pub status: ReviewStatus,
    pub priority: u32,
    pub round: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ReviewStatus {
    Pending,
    Approved,
    Rejected,
}
