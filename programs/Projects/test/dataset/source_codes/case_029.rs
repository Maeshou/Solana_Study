use anchor_lang::prelude::*;
declare_id!("Case0291111111111111111111111111111111111111");

#[program]
pub mod case029 {
    use super::*;
    pub fn execute_rent_nft(ctx: Context<RentNFTContext>) -> Result<()> {
        // Use Case 29: NFT レンタル（RentNFT）
        // Vulnerable: using UncheckedAccount where RentNFTAccount is expected
        msg!("Executing execute_rent_nft for NFT レンタル（RentNFT）");
        // Example logic (dummy operation)
        let mut acct_data = RentNFTAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
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
}