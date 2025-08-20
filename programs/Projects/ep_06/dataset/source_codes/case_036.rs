use anchor_lang::prelude::*;
declare_id!("CLEA0361111111111111111111111111111111111111");

#[program]
pub mod case036 {
    use super::*;
    pub fn execute_clearing(ctx: Context<ClearingContext>) -> Result<()> {
        // Derivatives clearing logic
        msg!("Clearing position");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClearingContext<'info> {
    /// CHECK: expecting ClearingAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting ClearingAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ClearingAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}