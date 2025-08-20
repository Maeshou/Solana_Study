use anchor_lang::prelude::*;

declare_id!("VulnInit2222222222222222222222222222222222");

#[program]
pub mod vuln_order {
    use super::*;

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
            sum += q as u64 * 100;
        }
        // 10以上で10%割引
        if total_qty > 10 {
            sum = sum * 90 / 100;
        }

        let order = &mut ctx.accounts.order_account;     // ← Init OK
        order.products   = product_ids.clone();
        order.quantity   = total_qty;
        order.created_at = Clock::get()?.unix_timestamp;

        let payment = &mut ctx.accounts.payment_account; // ← Init OK
        payment.payer  = ctx.accounts.user.key();
        payment.amount = sum;
        payment.paid   = false;

        // invoice は init されていない → 任意アドレス差し替え
        let invoice = &mut ctx.accounts.invoice;         // ← Init missing
        invoice.order     = order.key();
        invoice.due_dates = Vec::new();
        let start = Clock::get()?.unix_timestamp;
        for m in 1..=3 {
            invoice.due_dates.push(start + m * 30 * 86400);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateOrder<'info> {
    #[account(init, payer = user, space = 8 + (8*10) + 4 + 8)]
    pub order_account: Account<'info, OrderData>,
    #[account(init, payer = user, space = 8 + 32 + 8 + 1)]
    pub payment_account: Account<'info, PaymentData>,
    pub invoice: Account<'info, InvoiceData>,          // ← init がない
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
    pub due_dates: Vec<i64>,
}
