use anchor_lang::prelude::*;
declare_id!("BountyApp1111111111111111111111111111111");

/// バウンティ情報
#[account]
pub struct Bounty {
    pub organizer:   Pubkey,  // プログラム主催者
    pub description: String,  // 説明
    pub reward_pool: u64,     // 残り報酬プール
}

/// 提出記録
#[account]
pub struct Submission {
    pub submitter:   Pubkey,  // 提出者
    pub bounty:      Pubkey,  // Bounty.key()
    pub accepted:    bool,    // 採用フラグ
}

#[derive(Accounts)]
pub struct CreateBounty<'info> {
    #[account(init, payer = organizer, space = 8 + 32 + 4 + 256 + 8)]
    pub bounty:      Account<'info, Bounty>,
    #[account(mut)]
    pub organizer:   Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitWork<'info> {
    /// Bounty.organizer == organizer.key() は検証されない（不要）
    #[account(mut)]
    pub bounty:      Account<'info, Bounty>,

    /// Submission.bounty == bounty.key()、Submission.submitter == submitter.key() を検証
    #[account(
        init,
        payer = submitter,
        space = 8 + 32 + 32 + 1,
        has_one = bounty,
        has_one = submitter
    )]
    pub submission:  Account<'info, Submission>,

    #[account(mut)]
    pub submitter:   Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AcceptSubmission<'info> {
    /// Submission.submitter == signer.key() は検証されない
    #[account(mut)]
    pub submission:  Account<'info, Submission>,

    /// Submission.bounty == bounty.key() を検証
    #[account(mut, has_one = bounty)]
    pub bounty:      Account<'info, Bounty>,

    #[account(mut)]
    pub signer:      Signer<'info>,  // 主催者が署名
}

#[derive(Accounts)]
pub struct PayOut<'info> {
    /// Submission.bounty == bounty.key()、Submission.submitter == submitter.key() を検証
    #[account(mut, has_one = bounty, has_one = submitter)]
    pub submission:  Account<'info, Submission>,

    #[account(mut)]
    pub bounty:      Account<'info, Bounty>,

    #[account(mut)]
    pub submitter:   Signer<'info>,
}

#[program]
pub mod bounty_app {
    use super::*;

    pub fn create_bounty(
        ctx: Context<CreateBounty>,
        description: String,
        total_reward: u64
    ) -> Result<()> {
        let b = &mut ctx.accounts.bounty;
        b.organizer   = ctx.accounts.organizer.key();
        b.description = description;
        b.reward_pool = total_reward;
        Ok(())
    }

    pub fn submit_work(ctx: Context<SubmitWork>) -> Result<()> {
        let s = &mut ctx.accounts.submission;
        // 明示的にセット
        s.submitter = ctx.accounts.submitter.key();
        s.bounty    = ctx.accounts.bounty.key();
        s.accepted  = false;

        // 二重チェック
        require_keys_eq!(s.bounty, ctx.accounts.bounty.key(), BountyError::BountyMismatch);
        require_keys_eq!(s.submitter, ctx.accounts.submitter.key(), BountyError::SubmitterMismatch);
        Ok(())
    }

    pub fn accept_submission(ctx: Context<AcceptSubmission>) -> Result<()> {
        let s = &mut ctx.accounts.submission;
        // 主催者のみが実行する想定として、organizerチェックを手動で
        require_keys_eq!(ctx.accounts.bounty.organizer, ctx.accounts.signer.key(), BountyError::Unauthorized);

        s.accepted = true;
        Ok(())
    }

    pub fn pay_out(ctx: Context<PayOut>) -> Result<()> {
        let b = &mut ctx.accounts.bounty;
        let s = &ctx.accounts.submission;
        // 二重チェック
        require_keys_eq!(s.bounty, b.key(), BountyError::BountyMismatch);
        require_keys_eq!(s.submitter, ctx.accounts.submitter.key(), BountyError::SubmitterMismatch);
        require!(s.accepted, BountyError::NotAccepted);

        // 報酬支払い処理（lamports移動は省略）
        let amount =  b.reward_pool;
        b.reward_pool = b.reward_pool.checked_sub(amount).ok_or(BountyError::InsufficientFunds)?;
        Ok(())
    }
}

#[error_code]
pub enum BountyError {
    #[msg("Submission.bounty が Bounty と一致しません")]
    BountyMismatch,
    #[msg("Submission.submitter が署名者と一致しません")]
    SubmitterMismatch,
    #[msg("主催者のみ実行可能です")]
    Unauthorized,
    #[msg("まだ採用されていません")]
    NotAccepted,
    #[msg("報酬プールが不足しています")]
    InsufficientFunds,
}
