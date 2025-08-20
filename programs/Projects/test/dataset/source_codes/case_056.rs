use anchor_lang::prelude::*;
declare_id!("Case0561111111111111111111111111111111111111");

#[program]
pub mod case056 {
    use super::*;
    pub fn execute_on_chain_oracle(ctx: Context<OnChainOracleContext>) -> Result<()> {
        // Use Case 56: オンチェーン価格フィード（OnChainOracle）読む
        // Vulnerable: using UncheckedAccount where OnChainOracleAccount is expected
        msg!("Executing execute_on_chain_oracle for オンチェーン価格フィード（OnChainOracle）読む");
        // Example logic (dummy operation)
        let mut acct_data = OnChainOracleAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct OnChainOracleContext<'info> {
    /// CHECK: expecting OnChainOracleAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting OnChainOracleAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct OnChainOracleAccount {
    pub dummy: u64,
}