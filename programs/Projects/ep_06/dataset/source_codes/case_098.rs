use anchor_lang::prelude::*;
declare_id!("CONS0981111111111111111111111111111111111111");

#[program]
pub mod case098 {
    use super::*;
    pub fn execute_consumeenergycredit(ctx: Context<ConsumeEnergyCreditContext>) -> Result<()> {
        // Default context logic
        msg!("Case 98 executed");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ConsumeEnergyCreditContext<'info> {
    /// CHECK: expecting ConsumeEnergyCreditAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting ConsumeEnergyCreditAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ConsumeEnergyCreditAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}