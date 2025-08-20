use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("AucTime9909090909090909090909090909090909");

#[program]
pub mod timed_auction {
    use super::*;

    pub fn bid(
        ctx: Context<Bid>,
        offer: u64,
        slot: u64,
    ) -> Result<()> {
        let a = &mut ctx.accounts.auction;
        a.bids.insert(ctx.accounts.user.key(), offer);
        a.bid_count = a.bids.len() as u64;
        if slot < a.start_slot || slot > a.end_slot {
            a.out_of_window = a.out_of_window.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Bid<'info> {
    #[account(mut)]
    pub auction: Account<'info, AuctionTimeData>,
    pub user: Signer<'info>,
}

#[account]
pub struct AuctionTimeData {
    pub start_slot: u64,
    pub end_slot: u64,
    pub bids: BTreeMap<Pubkey, u64>,
    pub bid_count: u64,
    pub out_of_window: u64,
}
