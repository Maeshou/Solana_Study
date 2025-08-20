use anchor_lang::prelude::*;
declare_id!("Case0451111111111111111111111111111111111111");

#[program]
pub mod case045 {
    use super::*;
    pub fn execute_guild_reward(ctx: Context<GuildRewardContext>) -> Result<()> {
        // Use Case 45: ギルド報酬分配（GuildReward）
        // Vulnerable: using UncheckedAccount where GuildRewardAccount is expected
        msg!("Executing execute_guild_reward for ギルド報酬分配（GuildReward）");
        // Example logic (dummy operation)
        let mut acct_data = GuildRewardAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct GuildRewardContext<'info> {
    /// CHECK: expecting GuildRewardAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting GuildRewardAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct GuildRewardAccount {
    pub dummy: u64,
}