use anchor_lang::prelude::*;
declare_id!("SELL0221111111111111111111111111111111111111");

#[program]
pub mod case022 {
    use super::*;
    pub fn execute_sellitem(ctx: Context<SellItemContext>) -> Result<()> {
        // Game item logic
        msg!("Processing item transfer");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SellItemContext<'info> {
    /// CHECK: expecting SellItemAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting SellItemAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct SellItemAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}