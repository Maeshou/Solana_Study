use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("TradeDeal3333333333333333333333333333333333");

#[program]
pub mod trade_dealer {
    use super::*;

    pub fn negotiate(
        ctx: Context<Negotiate>,
        offer: u64,
        min_price: u64,
    ) -> Result<()> {
        let deal = &mut ctx.accounts.deal;
        let user = ctx.accounts.user.key();
        if offer >= min_price {
            // 成立
            deal.sold = true;
            deal.buyer = Some(user);
            deal.trade_count = deal.trade_count.saturating_add(1);
            deal.price = offer;
        } else {
            // 不成立
            deal.failed_offers.insert(user, offer);
            deal.fail_count = deal.fail_count.saturating_add(1);
            deal.next_min_price = deal.next_min_price.max(offer + 1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Negotiate<'info> {
    #[account(mut)]
    pub deal: Account<'info, DealData>,
    pub user: Signer<'info>,
}

#[account]
pub struct DealData {
    pub min_price: u64,
    pub sold: bool,
    pub buyer: Option<Pubkey>,
    pub price: u64,
    pub trade_count: u64,
    pub failed_offers: BTreeMap<Pubkey, u64>,
    pub fail_count: u64,
    pub next_min_price: u64,
}
