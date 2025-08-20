use anchor_lang::prelude::*;
declare_id!("Case0551111111111111111111111111111111111111");

#[program]
pub mod case055 {
    use super::*;
    pub fn execute_off_chain_oracle(ctx: Context<OffChainOracleContext>) -> Result<()> {
        // Use Case 55: オフチェーン価格フィード（OffChainOracle）更新
        // Vulnerable: using UncheckedAccount where OffChainOracleAccount is expected
        msg!("Executing execute_off_chain_oracle for オフチェーン価格フィード（OffChainOracle）更新");
        // Example logic (dummy operation)
        let mut acct_data = OffChainOracleAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct OffChainOracleContext<'info> {
    /// CHECK: expecting OffChainOracleAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting OffChainOracleAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct OffChainOracleAccount {
    pub dummy: u64,
}