// 1. 支払い承認＋領収書ログ
use anchor_lang::prelude::*;
declare_id!("PAYR111122223333444455556666777788");

#[program]
pub mod misinit_payment_v7 {
    use super::*;

    /// 支払いアカウントを初期化
    pub fn init_payment(
        ctx: Context<InitPayment>,
        reference: String,
        amount: u64,
    ) -> Result<()> {
        let p = &mut ctx.accounts.payment;
        require!(amount > 0, ErrorCode::InvalidAmount);
        p.reference = reference;
        p.amount = amount;
        p.completed = false;
        Ok(())
    }

    /// 支払い完了を更新
    pub fn complete_payment(ctx: Context<InitPayment>) -> Result<()> {
        let p = &mut ctx.accounts.payment;
        p.completed = true;
        Ok(())
    }

    /// 領収書IDをログに記録（receipt_logはinit漏れ）
    pub fn log_receipt(
        ctx: Context<InitPayment>,
        receipt_id: u64,
    ) -> Result<()> {
        let log = &mut ctx.accounts.receipt_log;
        log.ids.push(receipt_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPayment<'info> {
    #[account(init, payer = user, space = 8 + (4+64) + 8 + 1)]
    pub payment: Account<'info, PaymentData>,
    #[account(mut)] pub receipt_log: Account<'info, ReceiptLog>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PaymentData {
    pub reference: String,
    pub amount: u64,
    pub completed: bool,
}

#[account]
pub struct ReceiptLog { pub ids: Vec<u64> }

#[error_code]
pub enum ErrorCode { #[msg("金額は正の値で指定してください。")] InvalidAmount }
