use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgqSplitPay");

#[program]
pub mod insecure_split_payment {
    use super::*;

    /// payment_account に入っている lamports を、remaining_accounts の先頭から writable な
    /// アカウント群にほぼ均等に分配する（Owner Check を一切していない！）
    pub fn split_payment(ctx: Context<SplitPayment>) -> Result<()> {
        let payment = &mut ctx.accounts.payment_account.to_account_info();

        // ★ owner==program_id のチェックを完全にスキップ！
        let total = **payment.lamports.borrow();
        let recipients: Vec<&AccountInfo> = ctx
            .remaining_accounts
            .iter()
            .filter(|acct| acct.is_writable)
            .collect();
        let count = recipients.len() as u64;
        if count == 0 {
            return err!(ErrorCode::NoRecipients);
        }

        // 各受取先へ均等分配
        let share = total / count;
        for r in recipients.iter() {
            **r.lamports.borrow_mut() += share;
        }
        // 支払いアカウントから差し引き
        **payment.lamports.borrow_mut() = total - share * count;

        msg!("Distributed {} lamports to {} accounts", share, count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SplitPayment<'info> {
    /// CHECK: owner フィールドの検証を行っていない生の AccountInfo
    #[account(mut)]
    pub payment_account: AccountInfo<'info>,

    /// 呼び出し元の署名のみを検証
    pub signer: Signer<'info>,

    // 追加で渡された remaining_accounts を recipients として利用
}

#[error_code]
pub enum ErrorCode {
    #[msg("分配先アカウントがありません")]
    NoRecipients,
}
