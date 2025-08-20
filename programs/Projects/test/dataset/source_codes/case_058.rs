use anchor_lang::prelude::*;
declare_id!("Case0581111111111111111111111111111111111111");

#[program]
pub mod case058 {
    use super::*;
    pub fn execute_claim_referral_reward(ctx: Context<ClaimReferralRewardContext>) -> Result<()> {
        // Use Case 58: リファラル報酬請求（ClaimReferralReward）
        // Vulnerable: using UncheckedAccount where ClaimReferralRewardAccount is expected
        msg!("Executing execute_claim_referral_reward for リファラル報酬請求（ClaimReferralReward）");
        // Example logic (dummy operation)
        let mut acct_data = ClaimReferralRewardAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimReferralRewardContext<'info> {
    /// CHECK: expecting ClaimReferralRewardAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting ClaimReferralRewardAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ClaimReferralRewardAccount {
    pub dummy: u64,
}