use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgqBatchTr01");

#[program]
pub mod insecure_batch_transfer {
    use super::*;

    /// `reserve` から複数アカウントへ一括で lamports を送金する
    /// （Owner Check を一切行っていないため脆弱性あり）
    pub fn batch_transfer(
        ctx: Context<BatchTransfer>,
        amounts: Vec<u64>,
    ) -> Result<()> {
        // 生の AccountInfo で受け取っているため、reserve.owner == program_id の確認をスキップ
        let reserve = &mut ctx.accounts.reserve.to_account_info();
        let recipients = &ctx.remaining_accounts;

        // 合計必要額を計算
        let total: u64 = amounts.iter().copied().sum();
        if **reserve.lamports.borrow() < total {
            return err!(ErrorCode::InsufficientFunds);
        }

        // 各受取先に対応する金額を送金
        for (i, rcpt) in recipients.iter().enumerate() {
            let amt = amounts
                .get(i)
                .copied()
                .ok_or(ErrorCode::InvalidInput)?;
            **reserve.lamports.borrow_mut() -= amt;
            **rcpt.lamports.borrow_mut() += amt;
        }

        msg!("Batch transferred total {} lamports", total);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BatchTransfer<'info> {
    /// CHECK: owner==program_id の検証を行っていない生の AccountInfo
    #[account(mut)]
    pub reserve: AccountInfo<'info>,

    /// 呼び出し元が署名していることのみ検証
    pub initiator: Signer<'info>,

    // `remaining_accounts` に送金先を複数渡す
}

#[error_code]
pub enum ErrorCode {
    #[msg("reserve に十分な lamports がありません")]
    InsufficientFunds,
    #[msg("入力された金額リストと送金先アカウント数が一致しません")]
    InvalidInput,
}
