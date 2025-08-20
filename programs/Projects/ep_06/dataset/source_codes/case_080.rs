use anchor_lang::prelude::*;
declare_id!("ISSU0801111111111111111111111111111111111111");

#[program]
pub mod case080 {
    use super::*;
    pub fn execute_issuerec(ctx: Context<IssueRECContext>) -> Result<()> {
        // Default context logic
        msg!("Case 80 executed");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct IssueRECContext<'info> {
    /// CHECK: expecting IssueRECAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting IssueRECAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct IssueRECAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}