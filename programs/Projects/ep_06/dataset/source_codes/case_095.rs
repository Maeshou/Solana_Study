use anchor_lang::prelude::*;
declare_id!("USER0951111111111111111111111111111111111111");

#[program]
pub mod case095 {
    use super::*;
    pub fn execute_useregentoken(ctx: Context<UseRegenTokenContext>) -> Result<()> {
        // Default context logic
        msg!("Case 95 executed");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UseRegenTokenContext<'info> {
    /// CHECK: expecting UseRegenTokenAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting UseRegenTokenAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UseRegenTokenAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}