use anchor_lang::prelude::*;
declare_id!("EVAL0621111111111111111111111111111111111111");

#[program]
pub mod case062 {
    use super::*;
    pub fn execute_evaluaterwa(ctx: Context<EvaluateRWAContext>) -> Result<()> {
        // Default context logic
        msg!("Case 62 executed");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct EvaluateRWAContext<'info> {
    /// CHECK: expecting EvaluateRWAAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting EvaluateRWAAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct EvaluateRWAAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}