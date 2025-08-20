use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("OrderBk0777777777777777777777777777777777");

#[program]
pub mod order_book {
    use super::*;

    pub fn add_order(ctx: Context<ModifyBook>, price: u64) -> Result<()> {
        let ob = &mut ctx.accounts.book;
        ob.orders.entry(price).or_insert_with(Vec::new).push(ctx.accounts.user.key());
        Ok(())
    }

    pub fn remove_price_level(ctx: Context<ModifyBook>, price: u64) -> Result<()> {
        let ob = &mut ctx.accounts.book;
        ob.orders.remove(&price);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ModifyBook<'info> {
    #[account(mut)]
    pub book: Account<'info, OrderBookData>,
    pub user: Signer<'info>,
}

#[account]
pub struct OrderBookData {
    pub orders: BTreeMap<u64, Vec<Pubkey>>,
}
