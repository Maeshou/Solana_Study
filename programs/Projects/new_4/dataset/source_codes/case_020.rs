// 10. バイナリデータ＋処理フラグ
use anchor_lang::prelude::*;

declare_id!("Dat00000000000000000000000000000000");

#[program]
pub mod reinit_data_account_v2 {
    use super::*;

    // データを初めて書き込む
    pub fn init_data(
        ctx: Context<InitDataAccount>,
        data: Vec<u8>,
    ) -> Result<()> {
        let da = &mut ctx.accounts.data_account;
        da.data = data;
        da.processed = false;
        Ok(())
    }

    // 処理済みフラグを更新
    pub fn process_data(
        ctx: Context<InitDataAccount>,
    ) -> Result<()> {
        let da = &mut ctx.accounts.data_account;
        da.processed = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitDataAccount<'info> {
    #[account(mut)]
    pub data_account: Account<'info, DataAccount>,
    /// CHECK: メタ情報用、毎回上書き可能
    #[account(mut)]
    pub meta: AccountInfo<'info>,
}

#[account]
pub struct DataAccount {
    pub data: Vec<u8>,
    pub processed: bool,
}
