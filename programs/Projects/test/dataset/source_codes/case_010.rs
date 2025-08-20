use anchor_lang::prelude::*;
declare_id!("Case0101111111111111111111111111111111111111");

#[program]
pub mod case010 {
    use super::*;
    pub fn execute_mint_nft(ctx: Context<MintNFTContext>) -> Result<()> {
        // Use Case 10: NFT ミント（MintNFT）
        // Vulnerable: using UncheckedAccount where MintNFTAccount is expected
        msg!("Executing execute_mint_nft for NFT ミント（MintNFT）");
        // Example logic (dummy operation)
        let mut acct_data = MintNFTAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintNFTContext<'info> {
    /// CHECK: expecting MintNFTAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting MintNFTAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MintNFTAccount {
    pub dummy: u64,
}