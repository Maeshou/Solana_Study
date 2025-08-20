use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("BuyoutAuc111111111111111111111111111111111");

#[program]
pub mod buyout_auction {
    use super::*;

    /// 入札または即時購入
    pub fn place(ctx: Context<PlaceBid>, amount: u64) -> Result<()> {
        let auc = &mut ctx.accounts.auction;
        if amount >= auc.buyout_price {
            // 即時購入
            auc.sold_count = auc.sold_count.saturating_add(1);
            auc.buyer = Some(ctx.accounts.user.key());
            auc.final_price = amount;
        } else {
            // 通常入札
            auc.highest_bid = auc.highest_bid.max(amount);
            auc.bid_count = auc.bid_count.saturating_add(1);
            auc.bid_history.push((ctx.accounts.user.key(), amount));
            if auc.bid_history.len() > 20 {
                auc.bid_history.remove(0);
            }
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
    pub buyout_price: u64,
    pub highest_bid: u64,
    pub final_price: u64,
    pub sold_count: u64,
    pub bid_count: u64,
    pub buyer: Option<Pubkey>,
    pub bid_history: Vec<(Pubkey, u64)>,
}
