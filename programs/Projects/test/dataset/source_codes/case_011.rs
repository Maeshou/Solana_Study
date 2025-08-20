use anchor_lang::prelude::*;
declare_id!("Case0111111111111111111111111111111111111111");

#[program]
pub mod case011 {
    use super::*;
    pub fn execute_claim_nft(ctx: Context<ClaimNFTContext>) -> Result<()> {
        // Use Case 11: NFT 受取（ClaimNFT）
        // Vulnerable: using UncheckedAccount where ClaimNFTAccount is expected
        msg!("Executing execute_claim_nft for NFT 受取（ClaimNFT）");
        // Example logic (dummy operation)
        let mut acct_data = ClaimNFTAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimNFTContext<'info> {
    /// CHECK: expecting ClaimNFTAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting ClaimNFTAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ClaimNFTAccount {
    pub dummy: u64,
}