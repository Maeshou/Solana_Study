use anchor_lang::prelude::*;
declare_id!("RENT0291111111111111111111111111111111111111");

#[program]
pub mod case029 {
    use super::*;
    pub fn execute_rentnft(ctx: Context<RentNFTContext>) -> Result<()> {
        // NFT rent logic
        let mut rent = RentAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        rent.active = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RentNFTContext<'info> {
    /// CHECK: expecting RentNFTAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting RentNFTAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RentNFTAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}