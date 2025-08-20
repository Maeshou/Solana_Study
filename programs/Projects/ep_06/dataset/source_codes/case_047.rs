use anchor_lang::prelude::*;
declare_id!("LEAV0471111111111111111111111111111111111111");

#[program]
pub mod case047 {
    use super::*;
    pub fn execute_leaveguild(ctx: Context<LeaveGuildContext>) -> Result<()> {
        // Default context logic
        msg!("Case 47 executed");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LeaveGuildContext<'info> {
    /// CHECK: expecting LeaveGuildAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting LeaveGuildAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct LeaveGuildAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}