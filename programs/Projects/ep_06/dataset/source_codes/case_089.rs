use anchor_lang::prelude::*;
declare_id!("SERV0891111111111111111111111111111111111111");

#[program]
pub mod case089 {
    use super::*;
    pub fn execute_serverlessoraclequery(ctx: Context<ServerlessOracleQueryContext>) -> Result<()> {
        // Update oracle price
        let mut oracle = OracleAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        oracle.price = oracle.price.checked_add(5).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ServerlessOracleQueryContext<'info> {
    /// CHECK: expecting ServerlessOracleQueryAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting ServerlessOracleQueryAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ServerlessOracleQueryAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}