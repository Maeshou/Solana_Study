use anchor_lang::prelude::*;
declare_id!("ISSU0941111111111111111111111111111111111111");

#[program]
pub mod case094 {
    use super::*;
    pub fn execute_issueregentoken(ctx: Context<IssueRegenTokenContext>) -> Result<()> {
        // Default context logic
        msg!("Case 94 executed");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct IssueRegenTokenContext<'info> {
    /// CHECK: expecting IssueRegenTokenAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting IssueRegenTokenAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct IssueRegenTokenAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}