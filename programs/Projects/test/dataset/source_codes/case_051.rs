use anchor_lang::prelude::*;
declare_id!("Case0511111111111111111111111111111111111111");

#[program]
pub mod case051 {
    use super::*;
    pub fn execute_claim_reward(ctx: Context<ClaimRewardContext>) -> Result<()> {
        // Use Case 51: リワード請求（ClaimReward）
        // Vulnerable: using UncheckedAccount where ClaimRewardAccount is expected
        msg!("Executing execute_claim_reward for リワード請求（ClaimReward）");
        // Example logic (dummy operation)
        let mut acct_data = ClaimRewardAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimRewardContext<'info> {
    /// CHECK: expecting ClaimRewardAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting ClaimRewardAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ClaimRewardAccount {
    pub dummy: u64,
}