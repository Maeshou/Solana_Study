use anchor_lang::prelude::*;
declare_id!("Case0421111111111111111111111111111111111111");

#[program]
pub mod case042 {
    use super::*;
    pub fn execute_settle_auction(ctx: Context<SettleAuctionContext>) -> Result<()> {
        // Use Case 42: NFT オークション落札（SettleAuction）
        // Vulnerable: using UncheckedAccount where SettleAuctionAccount is expected
        msg!("Executing execute_settle_auction for NFT オークション落札（SettleAuction）");
        // Example logic (dummy operation)
        let mut acct_data = SettleAuctionAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        acct_data.dummy = acct_data.dummy.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SettleAuctionContext<'info> {
    /// CHECK: expecting SettleAuctionAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting SettleAuctionAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct SettleAuctionAccount {
    pub dummy: u64,
}