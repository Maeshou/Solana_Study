use anchor_lang::prelude::*;
declare_id!("JOIN0461111111111111111111111111111111111111");

#[program]
pub mod case046 {
    use super::*;
    pub fn execute_joinguild(ctx: Context<JoinGuildContext>) -> Result<()> {
        // Default context logic
        msg!("Case 46 executed");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct JoinGuildContext<'info> {
    /// CHECK: expecting JoinGuildAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting JoinGuildAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct JoinGuildAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}