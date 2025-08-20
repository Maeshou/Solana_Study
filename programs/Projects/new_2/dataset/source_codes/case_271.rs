// 5. 在庫削除（脆弱アカウント＋実行者検証）
use anchor_lang::prelude::*;

#[program]
pub mod inventory_remover {
    use super::*;
    pub fn remove(ctx: Context<Remove>) -> Result<()> {
        // 脆弱：任意アカウントの生データをゼロクリア
        let buf = &mut ctx.accounts.inv_data.try_borrow_mut_data()?;
        for byte in buf.iter_mut() {
            *byte = 0;
        }
        msg!("実行者 {} が削除実行", ctx.accounts.executor.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Remove<'info> {
    /// CHECK: 脆弱アカウント（検証なし）
    pub inv_data: AccountInfo<'info>,
    #[account(mut, has_one = executor)]
    pub inv_admin: Account<'info, RemoveAdmin>,
    pub executor: Signer<'info>,
}

#[account]
pub struct RemoveAdmin {
    pub executor: Pubkey,
}
