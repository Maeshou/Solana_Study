use anchor_lang::prelude::*;
declare_id!("BURN0521111111111111111111111111111111111111");

#[program]
pub mod case052 {
    use super::*;
    pub fn execute_burntoken(ctx: Context<BurnTokenContext>) -> Result<()> {
        // Default context logic
        msg!("Case 52 executed");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BurnTokenContext<'info> {
    /// CHECK: expecting BurnTokenAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting BurnTokenAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct BurnTokenAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}