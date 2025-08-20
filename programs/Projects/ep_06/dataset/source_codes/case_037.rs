use anchor_lang::prelude::*;
declare_id!("MARG0371111111111111111111111111111111111111");

#[program]
pub mod case037 {
    use super::*;
    pub fn execute_margincall(ctx: Context<MarginCallContext>) -> Result<()> {
        // Derivatives clearing logic
        msg!("Clearing position");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MarginCallContext<'info> {
    /// CHECK: expecting MarginCallAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting MarginCallAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MarginCallAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}