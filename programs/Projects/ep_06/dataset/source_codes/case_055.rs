use anchor_lang::prelude::*;
declare_id!("UPDA0551111111111111111111111111111111111111");

#[program]
pub mod case055 {
    use super::*;
    pub fn execute_updateoffchainoracle(ctx: Context<UpdateOffChainOracleContext>) -> Result<()> {
        // Update oracle price
        let mut oracle = OracleAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        oracle.price = oracle.price.checked_add(5).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateOffChainOracleContext<'info> {
    /// CHECK: expecting UpdateOffChainOracleAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting UpdateOffChainOracleAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UpdateOffChainOracleAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}