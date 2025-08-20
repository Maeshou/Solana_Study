use anchor_lang::prelude::*;
declare_id!("CALC0761111111111111111111111111111111111111");

#[program]
pub mod case076 {
    use super::*;
    pub fn execute_calculatepremium(ctx: Context<CalculatePremiumContext>) -> Result<()> {
        // Insurance premium logic
        msg!("Premium calculated");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CalculatePremiumContext<'info> {
    /// CHECK: expecting CalculatePremiumAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting CalculatePremiumAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CalculatePremiumAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}