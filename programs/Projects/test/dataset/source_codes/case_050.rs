use anchor_lang::prelude::*;
declare_id!("Case0501111111111111111111111111111111111111");

#[program]
pub mod case050 {
    use super::*;
    pub fn execute_join_reward_pool(ctx: Context<JoinRewardPoolContext>) -> Result<()> {
        // Use Case 50: リワードプールへの参加（JoinRewardPool）
        // Vulnerable: using UncheckedAccount where JoinRewardPoolAccount is expected
        msg!("Executing execute_join_reward_pool for リワードプールへの参加（JoinRewardPool）");
        // Example logic (dummy operation)
        let mut acct_data = JoinRewardPoolAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct JoinRewardPoolContext<'info> {
    /// CHECK: expecting JoinRewardPoolAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting JoinRewardPoolAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct JoinRewardPoolAccount {
    pub dummy: u64,
}