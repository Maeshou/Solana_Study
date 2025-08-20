use anchor_lang::prelude::*;
declare_id!("MktPlace111111111111111111111111111111111");

/// 注文情報
#[account]
pub struct Order {
    pub buyer:       Pubkey, // 購入者
    pub total_price: u64,    // 合計金額
    pub paid:        bool,   // 支払い済みフラグ
}

/// 決済記録
#[account]
pub struct Payment {
    pub payer:    Pubkey, // 支払い者
    pub order_id: Pubkey, // 本来は Order.key() と一致すべき
    pub amount:   u64,    // 支払額
}

#[derive(Accounts)]
pub struct CreateOrder<'info> {
    #[account(init, payer = buyer, space = 8 + 32 + 8 + 1)]
    pub order:         Account<'info, Order>,
    #[account(mut)]
    pub buyer:         Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MakePayment<'info> {
    /// Order.buyer == buyer.key() は検証される
    #[account(mut, has_one = buyer)]
    pub order:         Account<'info, Order>,

    /// Payment.order_id == order.key() の検証がない
    #[account(init, payer = buyer, space = 8 + 32 + 32 + 8)]
    pub payment:       Account<'info, Payment>,

    #[account(mut)]
    pub buyer:         Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConfirmPayment<'info> {
    /// Order.buyer == buyer.key() は検証される
    #[account(mut, has_one = buyer)]
    pub order:         Account<'info, Order>,

    /// Payment.order_id と order.key() の一致チェックがない
    #[account(mut)]
    pub payment:       Account<'info, Payment>,

    #[account(mut)]
    pub buyer:         Signer<'info>,
}

#[program]
pub mod marketplace_vuln {
    use super::*;

    /// 注文を作成
    pub fn create_order(ctx: Context<CreateOrder>, total_price: u64) -> Result<()> {
        let o = &mut ctx.accounts.order;
        o.buyer       = ctx.accounts.buyer.key();
        o.total_price = total_price;
        o.paid        = false;
        Ok(())
    }

    /// 支払いを記録
    pub fn make_payment(ctx: Context<MakePayment>, amount: u64) -> Result<()> {
        let p = &mut ctx.accounts.payment;
        let o = &ctx.accounts.order;
        // 脆弱性ポイント：
        // p.order_id = o.key(); と設定するだけで、
        // payment.order_id と order.key() の整合性検証をしていない
        p.payer    = ctx.accounts.buyer.key();
        p.order_id = o.key();
        p.amount   = amount;
        Ok(())
    }

    /// 支払いを確定
    pub fn confirm_payment(ctx: Context<ConfirmPayment>) -> Result<()> {
        let o = &mut ctx.accounts.order;
        let p = &ctx.accounts.payment;
        // 本来は必須：
        // require_keys_eq!(
        //     p.order_id,
        //     o.key(),
        //     ErrorCode::OrderMismatch
        // );
        // がないため、攻撃者が自分の偽 payment を渡して
        // 任意の order.paid を true にできてしまう
        o.paid = true;
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Payment が指定の Order と一致しません")]
    OrderMismatch,
}
