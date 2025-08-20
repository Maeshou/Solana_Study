// 5. 在庫補充＋価格履歴
use anchor_lang::prelude::*;
declare_id!("STCK111122223333444455556666777788");

#[program]
pub mod misinit_inventory_v7 {
    use super::*;

    pub fn init_stock(
        ctx: Context<InitStock>,
        item: String,
        quantity: u32,
    ) -> Result<()> {
        let s = &mut ctx.accounts.stock;
        s.item = item;
        s.quantity = quantity;
        Ok(())
    }

    pub fn restock(
        ctx: Context<InitStock>,
        add: u32,
    ) -> Result<()> {
        let s = &mut ctx.accounts.stock;
        s.quantity = s.quantity.checked_add(add).unwrap();
        Ok(())
    }

    pub fn log_price(
        ctx: Context<InitStock>,
        price: u64,
    ) -> Result<()> {
        let log = &mut ctx.accounts.price_log;
        log.prices.push(price);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStock<'info> {
    #[account(init, payer = manager, space = 8 + (4+32) + 4)] pub stock: Account<'info, StockData>,
    #[account(mut)] pub price_log: Account<'info, PriceLog>,
    #[account(mut)] pub manager: Signer<'info>, pub system_program: Program<'info, System>,
}

#[account]
pub struct StockData { pub item: String, pub quantity: u32 }
#[account]
pub struct PriceLog { pub prices: Vec<u64> }