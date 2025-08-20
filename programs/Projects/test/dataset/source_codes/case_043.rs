use anchor_lang::prelude::*;
declare_id!("Case0431111111111111111111111111111111111111");

#[program]
pub mod case043 {
    use super::*;
    pub fn execute_resell_nft(ctx: Context<ResellNFTContext>) -> Result<()> {
        // Use Case 43: NFT 二次販売（ResellNFT）
        // Vulnerable: using UncheckedAccount where ResellNFTAccount is expected
        msg!("Executing execute_resell_nft for NFT 二次販売（ResellNFT）");
        // Example logic (dummy operation)
        let mut acct_data = ResellNFTAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
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
}