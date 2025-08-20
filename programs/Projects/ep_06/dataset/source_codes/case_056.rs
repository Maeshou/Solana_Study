use anchor_lang::prelude::*;
declare_id!("READ0561111111111111111111111111111111111111");

#[program]
pub mod case056 {
    use super::*;
    pub fn execute_readonchainoracle(ctx: Context<ReadOnChainOracleContext>) -> Result<()> {
        // Update oracle price
        let mut oracle = OracleAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        oracle.price = oracle.price.checked_add(5).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ReadOnChainOracleContext<'info> {
    /// CHECK: expecting ReadOnChainOracleAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting ReadOnChainOracleAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ReadOnChainOracleAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}