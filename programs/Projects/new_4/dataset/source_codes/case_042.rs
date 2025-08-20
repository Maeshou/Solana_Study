// 7. 請求サイクル＋請求書履歴（Clockなし）
use anchor_lang::prelude::*;
declare_id!("BILLZZZZYYYYXXXXWWWWVVVVUUUUTTTT");

#[program]
pub mod misinit_billing_no_clock {
    use super::*;

    pub fn init_cycle(ctx: Context<InitCycle>, period: u32) -> Result<()> {
        let bc = &mut ctx.accounts.billing_cycle;
        bc.period = period;
        bc.counter = 0;
        Ok(())
    }

    pub fn issue_invoice(ctx: Context<InitCycle>, amount: u64) -> Result<()> {
        let inv = &mut ctx.accounts.invoice;
        inv.amount = amount;
        let hist = &mut ctx.accounts.invoice_history;
        hist.entries.push(amount);
        bc.counter = bc.counter.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitCycle<'info> {
    #[account(init, payer = manager, space = 8 + 4 + 1)] pub billing_cycle: Account<'info, BillingCycle>,
    #[account(mut)] pub invoice: Account<'info, InvoiceData>,
    #[account(mut)] pub invoice_history: Account<'info, InvoiceHistory>,
    #[account(mut)] pub manager: Signer<'info>, pub system_program: Program<'info, System>,
}

#[account]
pub struct BillingCycle { pub period:u32, pub counter:u8 }
#[account]
pub struct InvoiceData { pub amount:u64 }
#[account]
pub struct InvoiceHistory { pub entries: Vec<u64> }
