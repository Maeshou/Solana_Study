use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod basic_update {
    use super::*;
    pub fn update_data(ctx: Context<UpdateData>, new_data: u64) -> Result<()> {
        ctx.accounts.my_account.data = new_data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    // このアカウントのオーナーがカレントプログラムであることを自動でチェック
    #[account(mut, has_one = authority)]
    pub my_account: Account<'info, MyData>,
    pub authority: Signer<'info>,
}

#[account]
pub struct MyData {
    pub data: u64,
    pub authority: Pubkey,
}