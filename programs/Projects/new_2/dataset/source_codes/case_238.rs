use anchor_lang::prelude::*;

declare_id!("VulnEx37000000000000000000000000000000000037");

#[program]
pub mod example37 {
    pub fn place_bid(ctx: Context<Ctx37>, price: u64) -> Result<()> {
        // trace_log は所有者検証なし
        ctx.accounts.trace_log.data.borrow_mut().extend_from_slice(&price.to_le_bytes());
        // auction_account は has_one で seller 検証済み
        let auc = &mut ctx.accounts.auction_account;
        auc.highest_bid = auc.highest_bid.max(price);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx37<'info> {
    pub clock: Sysvar<'info, Clock>,
    #[account(mut, has_one = seller)]
    pub auction_account: Account<'info, AuctionAccount>,
    pub seller: Signer<'info>,
    #[account(mut)]
    pub trace_log: AccountInfo<'info>,
}

#[account]
pub struct AuctionAccount {
    pub seller: Pubkey,
    pub highest_bid: u64,
}
