use anchor_lang::prelude::*;
declare_id!("InvoicePay1111111111111111111111111111111");

/// 請求書情報
#[account]
pub struct Invoice {
    pub issuer:     Pubkey, // 請求者
    pub total_due:  u64,    // 請求金額
    pub amount_paid:u64,    // 入金済み合計
}

/// 入金記録
#[account]
pub struct Payment {
    pub payer:      Pubkey, // 支払者
    pub invoice:    Pubkey, // 本来は Invoice.key() と一致すべき
    pub amount:     u64,    // 今回入金額
}

#[derive(Accounts)]
pub struct CreateInvoice<'info> {
    #[account(init, payer = issuer, space = 8 + 32 + 8 + 8)]
    pub invoice:    Account<'info, Invoice>,
    #[account(mut)]
    pub issuer:     Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RecordPayment<'info> {
    /// Invoice.issuer == issuer.key() は検証される
    #[account(mut, has_one = issuer)]
    pub invoice:    Account<'info, Invoice>,

    /// Payment.invoice ⇔ invoice.key() の検証がない
    #[account(init, payer = payer, space = 8 + 32 + 32 + 8)]
    pub payment:    Account<'info, Payment>,

    #[account(mut)]
    pub payer:      Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ApplyPayment<'info> {
    /// Invoice.issuer == issuer.key() は検証される
    #[account(mut, has_one = issuer)]
    pub invoice:    Account<'info, Invoice>,

    /// Payment.invoice と invoice.key() の一致チェックがない
    #[account(mut)]
    pub payment:    Account<'info, Payment>,

    pub issuer:     Signer<'info>,
}

#[program]
pub mod invoice_vuln {
    use super::*;

    /// 新規請求書を作成
    pub fn create_invoice(ctx: Context<CreateInvoice>, total_due: u64) -> Result<()> {
        let inv = &mut ctx.accounts.invoice;
        inv.issuer      = ctx.accounts.issuer.key();
        inv.total_due   = total_due;
        inv.amount_paid = 0;
        Ok(())
    }

    /// 入金を記録
    pub fn record_payment(ctx: Context<RecordPayment>, amount: u64) -> Result<()> {
        let p = &mut ctx.accounts.payment;
        let i = &ctx.accounts.invoice;
        // 脆弱性ポイント：
        // p.invoice = i.key(); と設定するだけで、
        // Payment.invoice と Invoice.key() の検証がない
        p.payer   = ctx.accounts.payer.key();
        p.invoice = i.key();
        p.amount  = amount;
        Ok(())
    }

    /// 記録済み入金を請求書に反映
    pub fn apply_payment(ctx: Context<ApplyPayment>) -> Result<()> {
        let inv = &mut ctx.accounts.invoice;
        let pay = &ctx.accounts.payment;
        // 本来は必須：
        // require_keys_eq!(pay.invoice, inv.key(), ErrorCode::InvoiceMismatch);
        // がないため、攻撃者は任意の Payment アカウントを渡して
        // 他人の請求書の入金済み額を改竄できてしまう

        // 入金額を累積
        inv.amount_paid = inv.amount_paid
            .checked_add(pay.amount)
            .unwrap_or(inv.amount_paid);
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Payment が指定の Invoice と一致しません")]
    InvoiceMismatch,
}
