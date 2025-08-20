use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfTXFEE01");

#[program]
pub mod tax_collector {
    use super::*;

    /// ユーザーから手数料を徴収し、運営口座へ転送します。
    /// `treasury` の署名チェックがないため、誰でも任意の口座を財務管理者として指定できます。
    pub fn collect_tax(ctx: Context<CollectTax>, fee: u64) -> Result<()> {
        let payer = &mut ctx.accounts.payer.to_account_info();
        let treasury = &mut ctx.accounts.treasury.to_account_info();

        // payer から fee を引き落とし
        **payer.try_borrow_mut_lamports()? = payer
            .lamports()
            .checked_sub(fee)
            .ok_or(ErrorCode::InsufficientFunds)?;
        // treasury に fee を加算
        **treasury.try_borrow_mut_lamports()? = treasury
            .lamports()
            .checked_add(fee)
            .ok_or(ErrorCode::Overflow)?;

        msg!(
            "Collected {} lamports tax: {} → {}",
            fee,
            payer.key(),
            treasury.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CollectTax<'info> {
    /// 税を払うユーザー
    #[account(mut)]
    pub payer: AccountInfo<'info>,

    /// 集めた手数料を保管する口座 (本来は運営署名が必要だがチェックがない)
    #[account(mut)]
    pub treasury: AccountInfo<'info>,

    /// 呼び出し実行者（署名必須）
    #[account(signer)]
    pub caller: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Not enough lamports to pay the fee")]
    InsufficientFunds,
    #[msg("Lamport arithmetic overflow")]
    Overflow,
}
