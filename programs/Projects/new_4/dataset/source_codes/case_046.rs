// 1. E-Commerce注文＋請求履歴
use anchor_lang::prelude::*;
declare_id!("ECOM111122223333444455556666777788");

#[program]
pub mod misinit_ecommerce_v6 {
    use super::*;

    // 注文アカウントを初期化
    pub fn init_order(
        ctx: Context<InitOrder>,
        order_id: u64,
        total: u64,
    ) -> Result<()> {
        let order = &mut ctx.accounts.order;
        order.id = order_id;
        order.total = total;
        order.item_count = 0;
        Ok(())
    }

    // 商品を追加
    pub fn add_item(
        ctx: Context<InitOrder>,
        item_price: u64,
    ) -> Result<()> {
        let order = &mut ctx.accounts.order;
        require!(item_price > 0, ErrorCode::InvalidPrice);
        order.total = order.total.checked_add(item_price).unwrap();
        order.item_count = order.item_count.checked_add(1).unwrap();
        Ok(())
    }

    // 請求履歴に記録（履歴アカウントは init 漏れ）
    pub fn record_invoice(
        ctx: Context<InitOrder>,
        invoice_id: u64,
    ) -> Result<()> {
        let hist = &mut ctx.accounts.invoice_history;
        hist.entries.push(invoice_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitOrder<'info> {
    #[account(init, payer = user, space = 8 + 8 + 8 + 4)]
    pub order: Account<'info, OrderData>,
    #[account(mut)] pub invoice_history: Account<'info, InvoiceHistory>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct OrderData { pub id: u64, pub total: u64, pub item_count: u32 }
#[account]
pub struct InvoiceHistory { pub entries: Vec<u64> }

#[error_code]
pub enum ErrorCode { #[msg("価格は正の値である必要があります。")] InvalidPrice }
