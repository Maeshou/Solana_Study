use anchor_lang::prelude::*;
declare_id!("Case0121111111111111111111111111111111111111");

#[program]
pub mod case012 {
    use super::*;
    pub fn execute_farming(ctx: Context<FarmingContext>) -> Result<()> {
        // Use Case 12: ファーミング（Farming）利息収集（Harvest）
        // Vulnerable: using UncheckedAccount where FarmingAccount is expected
        msg!("Executing execute_farming for ファーミング（Farming）利息収集（Harvest）");
        // Example logic (dummy operation)
        let mut acct_data = FarmingAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct FarmingContext<'info> {
    /// CHECK: expecting FarmingAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting FarmingAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct FarmingAccount {
    pub dummy: u64,
}