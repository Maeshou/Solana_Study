use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfA15mvTWf");

#[program]
pub mod fee_enforced_writer_003 {
    use super::*;

    pub fn write_with_fee(ctx: Context<Ctx003>, val: u64) -> Result<()> {
        // 書き込み前に fee を徴収
        let fee = 500;
        let payer = ctx.accounts.payer.to_account_info();
        let receiver = ctx.accounts.fee_receiver.to_account_info();

        // lamports 移動 (分岐なし)
        **payer.try_borrow_mut_lamports()? -= fee;
        **receiver.try_borrow_mut_lamports()? += fee;

        // 値の更新
        ctx.accounts.storage.data = val;
        Ok(())
    }

    pub fn read(ctx: Context<Ctx003>) -> Result<()> {
        msg!("Stored value: {}", ctx.accounts.storage.data);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx003<'info> {
    #[account(mut)]
    pub payer: Signer<'info>, // 実際に fee を払うアカウント

    #[account(mut)]
    pub fee_receiver: AccountInfo<'info>, // 手数料受け取り先

    #[account(mut, has_one = authority)]
    pub storage: Account<'info, Storage003>,

    pub authority: Signer<'info>,
}

#[account]
pub struct Storage003 {
    pub authority: Pubkey,
    pub data: u64,
}
