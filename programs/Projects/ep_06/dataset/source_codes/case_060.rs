use anchor_lang::prelude::*;
declare_id!("CONS0601111111111111111111111111111111111111");

#[program]
pub mod case060 {
    use super::*;
    pub fn execute_consumepoints(ctx: Context<ConsumePointsContext>) -> Result<()> {
        // Default context logic
        msg!("Case 60 executed");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ConsumePointsContext<'info> {
    /// CHECK: expecting ConsumePointsAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting ConsumePointsAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ConsumePointsAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}