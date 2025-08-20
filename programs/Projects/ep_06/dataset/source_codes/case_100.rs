use anchor_lang::prelude::*;
declare_id!("EXEC1001111111111111111111111111111111111111");

#[program]
pub mod case100 {
    use super::*;
    pub fn execute_executeammswap(ctx: Context<ExecuteAMMSwapContext>) -> Result<()> {
        // AMM swap or price update logic
        msg!("Swap executed or price updated");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExecuteAMMSwapContext<'info> {
    /// CHECK: expecting ExecuteAMMSwapAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting ExecuteAMMSwapAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ExecuteAMMSwapAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}