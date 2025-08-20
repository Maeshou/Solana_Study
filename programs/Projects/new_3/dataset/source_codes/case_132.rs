use anchor_lang::prelude::*;

#[account]
pub struct Order {
    pub buyer:  Pubkey,  // この buyer と署名者が一致するかは has_one でチェック
    pub amount: u64,
}

#[account]
pub struct Payment {
    pub payer:    Pubkey,
    pub order_id: u64,
}

#[derive(Accounts)]
pub struct ConfirmPayment<'info> {
    /// Order.buyer == buyer.key() は検証される
    #[account(mut, has_one = buyer)]
    pub order: Account<'info, Order>,

    /// しかし、この payment はただの mut アカウント。
    /// Order の PDA か、同一レコードを指しているかは一切チェックされない。
    #[account(mut)]
    pub payment: Account<'info, Payment>,

    /// 署名者チェックはあるが、payment.payer の検証すら入っていない
    pub buyer: Signer<'info>,
}

#[program]
pub mod account_matching_vuln {
    use super::*;

    pub fn confirm_payment(ctx: Context<ConfirmPayment>) -> Result<()> {
        // 本来は以下のいずれかで検証すべき
        // 1) require_keys_eq!(ctx.accounts.payment.key(), ctx.accounts.order.key(), MyError::AccountMismatch);
        // 2) #[account(address = order.key())] を payment に付与
        //
        // どちらもないため、攻撃者は自分で別の Payment アカウントを用意し、
        // payer フィールドに buyer.key() をセットして渡すことで
        // 好き勝手に上書きできてしまう。

        ctx.accounts.payment.payer = ctx.accounts.buyer.key();
        Ok(())
    }
}

#[error_code]
pub enum MyError {
    #[msg("Order と Payment のアカウントが一致しません")]
    AccountMismatch,
}
