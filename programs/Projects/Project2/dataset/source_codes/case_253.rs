use anchor_lang::prelude::*;

declare_id!("AucMin0222222222222222222222222222222222");

#[program]
pub mod auction_reserve {
    use super::*;

    pub fn place_bid(ctx: Context<Place>, amount: u64) -> Result<()> {
        let a = &mut ctx.accounts.auction;
        if amount >= a.reserve_price {
            a.highest_bid = amount;
            a.bid_count = a.bid_count.saturating_add(1);
        } else {
            a.rejected_bids = a.rejected_bids.saturating_add(1);
            a.min_raise = a.min_raise.max(a.reserve_price.saturating_sub(amount));
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Place<'info> {
    #[account(mut)]
    pub auction: Account<'info, ReserveAuction>,
    pub bidder: Signer<'info>,
}

#[account]
pub struct ReserveAuction {
    pub reserve_price: u64,
    pub highest_bid: u64,
    pub bid_count: u64,
    pub rejected_bids: u64,
    pub min_raise: u64,
}
