use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfESCRO04");

#[program]
pub mod escrow_release {
    use super::*;

    /// エスクロー口座からベネフィシャリーへ任意の金額をリリースします。
    /// `manager` には署名チェックが入っておらず、誰でも呼び出せる脆弱性があります。
    pub fn release_funds(
        ctx: Context<ReleaseFunds>,
        amount: u64,  // リリースする lamports
    ) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow.to_account_info();
        let beneficiary = &mut ctx.accounts.beneficiary.to_account_info();

        // manager の署名チェックがないため、誰でもこの命令を実行できてしまいます
        **escrow.try_borrow_mut_lamports()? = escrow
            .lamports()
            .checked_sub(amount)
            .ok_or(ErrorCode::InsufficientFunds)?;
        **beneficiary.try_borrow_mut_lamports()? = beneficiary
            .lamports()
            .checked_add(amount)
            .ok_or(ErrorCode::Overflow)?;

        msg!(
            "Released {} lamports from {} to {}",
            amount,
            escrow.key(),
            beneficiary.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ReleaseFunds<'info> {
    /// 資金を保持するエスクロー口座
    #[account(mut)]
    pub escrow: AccountInfo<'info>,

    /// 資金を受け取るベネフィシャリー口座
    #[account(mut)]
    pub beneficiary: AccountInfo<'info>,

    /// 本来は管理者であるべきアカウントですが、署名チェックがありません
    pub manager: AccountInfo<'info>,

    /// トランザクションの実行者（必ず署名が必要）
    #[account(signer)]
    pub authority: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Escrow account has insufficient funds")]
    InsufficientFunds,
    #[msg("Lamport arithmetic overflow")]
    Overflow,
}
