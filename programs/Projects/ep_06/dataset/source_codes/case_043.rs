use anchor_lang::prelude::*;
declare_id!("RESE0431111111111111111111111111111111111111");

#[program]
pub mod case043 {
    use super::*;
    pub fn execute_resellnft(ctx: Context<ResellNFTContext>) -> Result<()> {
        // Default context logic
        msg!("Case 43 executed");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ResellNFTContext<'info> {
    /// CHECK: expecting ResellNFTAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting ResellNFTAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ResellNFTAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}