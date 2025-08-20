use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgq1bTrn9tXw");

#[program]
pub mod insecure_owner_check {
    use super::*;

    /// データを上書きするだけのシンプルなインストラクション
    pub fn set_data(ctx: Context<SetData>, new_value: u64) -> Result<()> {
        // ★ ここで所有者チェックをしていないため、任意の外部アカウントのデータを
        //    読み書きされるリスクがある
        let mut data: DataStruct = DataStruct::try_from_slice(
            &ctx.accounts.data_account.data.borrow()
        )?;
        data.value = new_value;
        data.serialize(&mut &mut ctx.accounts.data_account.data.borrow_mut()[..])?;
        Ok(())
    }
}

/// `data_account` が本プログラムのアカウントであることを検証していない！
#[derive(Accounts)]
pub struct SetData<'info> {
    /// CHECK: 所有者チェック（owner == program_id）を行っていない
    #[account(mut)]
    pub data_account: AccountInfo<'info>,

    /// この操作を行う権限を持つ署名者だが、
    /// data_account の持ち主かどうかは検証していない
    pub authority: Signer<'info>,
}

#[account]
pub struct DataStruct {
    pub value: u64,
}
