use anchor_lang::prelude::*;
declare_id!("Case0411111111111111111111111111111111111111");

#[program]
pub mod case041 {
    use super::*;
    pub fn execute_place_bid(ctx: Context<PlaceBidContext>) -> Result<()> {
        // Use Case 41: NFT オークション入札（PlaceBid）
        // Vulnerable: using UncheckedAccount where PlaceBidAccount is expected
        msg!("Executing execute_place_bid for NFT オークション入札（PlaceBid）");
        // Example logic (dummy operation)
        let mut acct_data = PlaceBidAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PlaceBidContext<'info> {
    /// CHECK: expecting PlaceBidAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting PlaceBidAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PlaceBidAccount {
    pub dummy: u64,
}