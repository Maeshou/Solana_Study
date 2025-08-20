use anchor_lang::prelude::*;
declare_id!("Case0041111111111111111111111111111111111111");

#[program]
pub mod case004 {
    use super::*;
    pub fn execute_staking(ctx: Context<StakingContext>) -> Result<()> {
        // Use Case 4: ステーキング（Staking）登録
        // Vulnerable: using UncheckedAccount where StakingAccount is expected
        msg!("Executing execute_staking for ステーキング（Staking）登録");
        // Example logic (dummy operation)
        let mut acct_data = StakingAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StakingContext<'info> {
    /// CHECK: expecting StakingAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting StakingAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct StakingAccount {
    pub dummy: u64,
}