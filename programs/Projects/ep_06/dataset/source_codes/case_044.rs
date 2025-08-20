use anchor_lang::prelude::*;
declare_id!("CRAF0441111111111111111111111111111111111111");

#[program]
pub mod case044 {
    use super::*;
    pub fn execute_craftitem(ctx: Context<CraftItemContext>) -> Result<()> {
        // Default context logic
        msg!("Case 44 executed");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CraftItemContext<'info> {
    /// CHECK: expecting CraftItemAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting CraftItemAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CraftItemAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}