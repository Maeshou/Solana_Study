use anchor_lang::prelude::*;
declare_id!("Case0891111111111111111111111111111111111111");

#[program]
pub mod case089 {
    use super::*;
    pub fn execute_serverless_oracle(ctx: Context<ServerlessOracleContext>) -> Result<()> {
        // Use Case 89: サーバーレス・オラクル（ServerlessOracle）クエリ
        // Vulnerable: using UncheckedAccount where ServerlessOracleAccount is expected
        msg!("Executing execute_serverless_oracle for サーバーレス・オラクル（ServerlessOracle）クエリ");
        // Example logic (dummy operation)
        let mut acct_data = ServerlessOracleAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ServerlessOracleContext<'info> {
    /// CHECK: expecting ServerlessOracleAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting ServerlessOracleAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ServerlessOracleAccount {
    pub dummy: u64,
}