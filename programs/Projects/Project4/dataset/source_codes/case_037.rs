use anchor_lang::prelude::*;

declare_id!("InitAll2222222222222222222222222222222222");

#[program]
pub mod multi_init2 {
    use super::*;

    // 複数商品＋数量をループで合計し、割引適用／複数回分割請求を生成
    pub fn create_order(
        ctx: Context<CreateOrder>,
        product_ids: Vec<u64>,
        quantities: Vec<u32>,
    ) -> Result<()> {
        // 合計数量・金額を計算
        let mut total_qty = 0u32;
        let mut sum = 0u64;
        for (i, &pid) in product_ids.iter().enumerate() {
            let q = *quantities.get(i).unwrap_or(&0);
            total_qty += q;
            sum += q as u64 * 100; // 仮価格
        }
        // 10点以上なら10%割引
        if total_qty > 10 {
            sum = sum * 90 / 100;
        }

        // OrderData
        let order = &mut ctx.accounts.order_account;
        order.products = product_ids.clone();
        order.quantity = total_qty;
        order.created_at = Clock::get()?.unix_timestamp;

        // PaymentData
        let payment = &mut ctx.accounts.payment_account;
        payment.payer = ctx.accounts.user.key();
        payment.amount = sum;
        payment.paid = false;

        // InvoiceData：3回分割払いスケジュールをループで生成
        let invoice = &mut ctx.accounts.invoice;
        invoice.order = order.key();
        let start = Clock::get()?.unix_timestamp;
        invoice.schedule = Vec::new();
        for m in 1..=3 {
            invoice.schedule.push(start + (m as i64) * 30 * 86400);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateOrder<'info> {
    #[account(init, payer = user, space = 8 + (8*10) + 4 + 8 + 8)]
    pub order_account: Account<'info, OrderData>,
    #[account(init, payer = user, space = 8 + 32 + 8 + 1)]
    pub payment_account: Account<'info, PaymentData>,
    #[account(init, payer = user, space = 8 + 32 + 4 + (8*3))]
    pub invoice: Account<'info, InvoiceData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct OrderData {
    pub products: Vec<u64>,
    pub quantity: u32,
    pub created_at: i64,
}

#[account]
pub struct PaymentData {
    pub payer: Pubkey,
    pub amount: u64,
    pub paid: bool,
}

#[account]
pub struct InvoiceData {
    pub order: Pubkey,
    pub schedule: Vec<i64>,
}
