use anchor_lang::prelude::*;

declare_id!("SafeMulti2222222222222222222222222222222222");

#[program]
pub mod safe_order {
    use super::*;

    // order, payment, invoice をすべて初期化
    pub fn create_order(
        ctx: Context<CreateOrder>,
        product_ids: Vec<u64>,
        quantities: Vec<u32>,
    ) -> Result<()> {
        let mut total_qty = 0u32;
        let mut sum = 0u64;
        for (i, &pid) in product_ids.iter().enumerate() {
            let q = *quantities.get(i).unwrap_or(&0);
            total_qty += q;
            sum += q as u64 * 100;
        }
        if total_qty > 10 {
            sum = sum * 90 / 100;
        }

        let order = &mut ctx.accounts.order;
        order.products = product_ids.clone();
        order.quantity = total_qty;
        order.created_at = Clock::get()?.unix_timestamp;

        let payment = &mut ctx.accounts.payment;
        payment.payer = ctx.accounts.user.key();
        payment.amount = sum;
        payment.paid = false;

        let invoice = &mut ctx.accounts.invoice;
        invoice.order = order.key();
        invoice.due_dates = Vec::new();
        let start = order.created_at;
        for m in 1..=3 {
            invoice.due_dates.push(start + m * 30 * 86400);
        }

        Ok(())
    }

    // invoice のみ更新
    pub fn schedule_invoice(ctx: Context<ScheduleInvoice>) -> Result<()> {
        let order = &ctx.accounts.order;
        let invoice = &mut ctx.accounts.invoice;
        invoice.due_dates.push(Clock::get()?.unix_timestamp + 30 * 86400);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateOrder<'info> {
    #[account(init, payer = user, space = 8 + (8*10) + 4 + 8)]
    pub order: Account<'info, OrderData>,
    #[account(init, payer = user, space = 8 + 32 + 8 + 1)]
    pub payment: Account<'info, PaymentData>,
    #[account(init, payer = user, space = 8 + 32 + (8*4))]
    pub invoice: Account<'info, InvoiceData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ScheduleInvoice<'info> {
    #[account(mut)] pub invoice: Account<'info, InvoiceData>,
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
