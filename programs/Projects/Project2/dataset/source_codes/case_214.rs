use anchor_lang::prelude::*;

declare_id!("MktExa3333333333333333333333333333333333");

#[program]
pub mod market_extra {
    use super::*;

    pub fn finalize(ctx: Context<Finalize>) -> Result<()> {
        let l = &mut ctx.accounts.listing;
        if l.highest_bid >= l.ask_price {
            // 売却成立
            l.sold = true;
            // 売却価格記録
            l.sale_price = l.highest_bid;
            // 約定手数料を計算
            l.fee = (l.sale_price / 100) * 2;
        } else {
            // 売却失敗
            l.active = false;
            // 再出品カウンタを増やす
            l.relist_count = l.relist_count.saturating_add(1);
            // 入札者への返却額計算
            l.refund_amount = l.highest_bid;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Finalize<'info> {
    #[account(mut)]
    pub listing: Account<'info, ListingExtraData>,
}

#[account]
pub struct ListingExtraData {
    pub ask_price: u64,
    pub highest_bid: u64,
    pub sold: bool,
    pub active: bool,
    pub sale_price: u64,
    pub fee: u64,
    pub relist_count: u64,
    pub refund_amount: u64,
}
