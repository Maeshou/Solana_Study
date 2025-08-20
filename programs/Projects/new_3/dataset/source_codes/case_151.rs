use anchor_lang::prelude::*;
declare_id!("BountyPr1111111111111111111111111111111111");

/// バウンティ案件情報
#[account]
pub struct Bounty {
    pub owner:        Pubkey, // バウンティ作成者
    pub reward:       u64,    // 採用時の報酬額（lamports）
    pub submissions:  u64,    // 提出数
}

/// 提出情報
#[account]
pub struct Submission {
    pub submitter: Pubkey, // 提出者
    pub bounty_id: Pubkey, // 本来は Bounty.key() と一致すべき
    pub accepted:  bool,   // 採用済みフラグ
}

/// バウンティ作成
#[derive(Accounts)]
pub struct CreateBounty<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8)]
    pub bounty:       Account<'info, Bounty>,
    #[account(mut)]
    pub owner:        Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// 提出受付
#[derive(Accounts)]
pub struct SubmitBounty<'info> {
    /// Bounty.owner == submitter.key() の検証は不要
    #[account(mut)]
    pub bounty:       Account<'info, Bounty>,

    #[account(init, payer = submitter, space = 8 + 32 + 32 + 1)]
    pub submission:   Account<'info, Submission>,

    #[account(mut)]
    pub submitter:    Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// 提出の採用
#[derive(Accounts)]
pub struct AcceptSubmission<'info> {
    /// Bounty.owner == owner.key() は検証される
    #[account(mut, has_one = owner)]
    pub bounty:       Account<'info, Bounty>,

    /// Submission.bounty_id == bounty.key() の検証がないため、
    /// 別のバウンティ用の Submission を渡しても通ってしまう
    #[account(mut)]
    pub submission:   Account<'info, Submission>,

    pub owner:        Signer<'info>,
}

#[program]
pub mod bounty_vuln {
    use super::*;

    /// 新しいバウンティを作成
    pub fn create_bounty(ctx: Context<CreateBounty>, reward: u64) -> Result<()> {
        let b = &mut ctx.accounts.bounty;
        b.owner       = ctx.accounts.owner.key();
        b.reward      = reward;
        b.submissions = 0;
        msg!("Created bounty {} with reward {}", b.key(), b.reward);
        Ok(())
    }

    /// バウンティへの提出
    pub fn submit(ctx: Context<SubmitBounty>, details: String) -> Result<()> {
        let b  = &mut ctx.accounts.bounty;
        let s  = &mut ctx.accounts.submission;
        // 脆弱性ポイント：
        // s.bounty_id = b.key(); と設定するだけで、
        // Submission.bounty_id と Bounty.key() の一致検証は行われない
        s.submitter = ctx.accounts.submitter.key();
        s.bounty_id = b.key();
        s.accepted  = false;
        b.submissions = b.submissions.checked_add(1).unwrap();
        msg!(
            "Submission {} recorded for bounty {} (total submissions: {})",
            s.key(),
            b.key(),
            b.submissions
        );
        Ok(())
    }

    /// 提出を採用して報酬支払い
    pub fn accept(ctx: Context<AcceptSubmission>) -> Result<()> {
        let b = &mut ctx.accounts.bounty;
        let s = &mut ctx.accounts.submission;
        // 本来は必須：
        // require_keys_eq!(
        //     s.bounty_id,
        //     b.key(),
        //     ErrorCode::BountyMismatch
        // );
        s.accepted = true;
        **b.to_account_info().try_borrow_mut_lamports()? -= b.reward;
        **ctx.accounts.owner.to_account_info().try_borrow_mut_lamports()? += b.reward;
        msg!(
            "Submission {} accepted for bounty {}; {} paid {} lamports",
            s.key(),
            b.key(),
            b.owner,
            b.reward
        );
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Submission が指定の Bounty と一致しません")]
    BountyMismatch,
}
