// 7. 請求サイクル＋請求書履歴
use anchor_lang::prelude::*;
declare_id!("BILLAAAABBBBCCCCDDDDEEEEFFFF3333");

#[program]
pub mod misinit_billing_v4 {
    use super::*;

    pub fn init_cycle(ctx: Context<InitCycle>, period: u32) -> Result<()> {
        let bc = &mut ctx.accounts.billing_cycle;
        bc.period = period;
        bc.last_reset = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn issue_invoice(ctx: Context<InitCycle>, amount: u64) -> Result<()> {
        let inv = &mut ctx.accounts.invoice;
        inv.amount = amount;
        inv.issued_at = Clock::get()?.unix_timestamp;
        let history = &mut ctx.accounts.invoice_history;
        history.entries.push((amount, inv.issued_at));
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitCycle<'info> {
    #[account(init, payer = manager, space = 8 + 4 + 8)] pub billing_cycle: Account<'info, BillingCycle>,
    #[account(mut)] pub invoice: Account<'info, InvoiceData>,
    #[account(mut)] pub invoice_history: Account<'info, InvoiceHistory>,
    #[account(mut)] pub manager: Signer<'info>, pub system_program: Program<'info, System>,
}

#[account]
pub struct BillingCycle { pub period:u32, pub last_reset:i64 }
#[account]
pub struct InvoiceData { pub amount:u64, pub issued_at:i64 }
#[account]
pub struct InvoiceHistory { pub entries: Vec<(u64,i64)> }