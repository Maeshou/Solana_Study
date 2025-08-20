use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgqCloseAcct1");

#[program]
pub mod insecure_account_close {
    use super::*;

    /// 任意のアカウントをクローズして lamports を移動、データをゼロクリアする
    pub fn close_data(ctx: Context<CloseData>) -> Result<()> {
        let data_acct = &mut ctx.accounts.data_account.to_account_info();
        let recipient = &mut ctx.accounts.recipient.to_account_info();

        // ★ オーナーチェックなし ★
        // 任意のアカウント(data_account)が指定されても処理が通ってしまう

        // 1. lamports を全額 recipient に移動
        let balance = **data_acct.lamports.borrow();
        **recipient.lamports.borrow_mut() += balance;
        **data_acct.lamports.borrow_mut() = 0;

        // 2. data_account のデータ領域をゼロ埋め
        let data = &mut *data_acct.data.borrow_mut();
        for byte in data.iter_mut() {
            *byte = 0;
        }

        msg!("Account closed and data wiped");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseData<'info> {
    /// CHECK: owner フィールドを検証していない
    #[account(mut)]
    pub data_account: AccountInfo<'info>,

    /// CHECK: 送金先アカウント（署名不要／型検証なし）
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
}
