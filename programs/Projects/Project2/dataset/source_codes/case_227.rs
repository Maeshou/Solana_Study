use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("AuctBid6666666666666666666666666666666666");

#[program]
pub mod auction_bidder {
    use super::*;

    pub fn place_bid(
        ctx: Context<PlaceBid>,
        amount: u64,
    ) -> Result<()> {
        let a = &mut ctx.accounts.auction;
        let bidder = ctx.accounts.user.key();
        if amount > a.highest_bid {
            // 新高値
            if let Some(prev) = a.highest_bidder {
                a.refund_map.insert(prev, a.highest_bid);
            }
            a.highest_bid = amount;
            a.highest_bidder = Some(bidder);
            a.bid_count = a.bid_count.saturating_add(1);
        } else {
            // 失格入札
            a.reject_map
                .entry(bidder)
                .and_modify(|c| *c += 1)
                .or_insert(1);
            a.reject_count = a.reject_count.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PlaceBid<'info> {
    #[account(mut)]
    pub auction: Account<'info, AuctionData>,
    pub user: Signer<'info>,
}

#[account]
pub struct AuctionData {
    pub highest_bid: u64,
    pub highest_bidder: Option<Pubkey>,
    pub bid_count: u64,
    pub refund_map: BTreeMap<Pubkey, u64>,
    pub reject_map: BTreeMap<Pubkey, u64>,
    pub reject_count: u64,
}
