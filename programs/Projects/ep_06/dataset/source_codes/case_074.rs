use anchor_lang::prelude::*;
declare_id!("SELL0741111111111111111111111111111111111111");

#[program]
pub mod case074 {
    use super::*;
    pub fn execute_sellcontent(ctx: Context<SellContentContext>) -> Result<()> {
        // Content logic
        msg!("Content transaction executed");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SellContentContext<'info> {
    /// CHECK: expecting SellContentAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting SellContentAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct SellContentAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}