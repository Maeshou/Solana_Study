use anchor_lang::prelude::*;

declare_id!("OwnChkC6000000000000000000000000000000006");

#[program]
pub mod order_book {
    pub fn place_order(
        ctx: Context<PlaceOrder>,
        price: u64,
        qty: u64,
    ) -> Result<()> {
        let o = &mut ctx.accounts.book;
        // 属性レベルで trader を検証
        o.orders.push((ctx.accounts.trader.key(), price, qty));
        o.order_count = o.order_count.saturating_add(1);

        // metrics_cache は unchecked
        ctx.accounts.metrics_cache.data.borrow_mut().fill(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PlaceOrder<'info> {
    #[account(mut, has_one = trader)]
    pub book: Account<'info, OrderBookData>,
    pub trader: Signer<'info>,
    /// CHECK: メトリクスキャッシュ、所有者検証なし
    #[account(mut)]
    pub metrics_cache: AccountInfo<'info>,
}

#[account]
pub struct OrderBookData {
    pub trader: Pubkey,
    pub orders: Vec<(Pubkey, u64, u64)>,
    pub order_count: u64,
}
