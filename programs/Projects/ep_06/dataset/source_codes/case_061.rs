use anchor_lang::prelude::*;
declare_id!("COLL0611111111111111111111111111111111111111");

#[program]
pub mod case061 {
    use super::*;
    pub fn execute_collateralizerwa(ctx: Context<CollateralizeRWAContext>) -> Result<()> {
        // Default context logic
        msg!("Case 61 executed");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CollateralizeRWAContext<'info> {
    /// CHECK: expecting CollateralizeRWAAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting CollateralizeRWAAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CollateralizeRWAAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}