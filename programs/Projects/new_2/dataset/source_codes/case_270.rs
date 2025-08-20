// 4. 在庫追加（脆弱アカウント＋在庫管理検証）
use anchor_lang::prelude::*;

#[program]
pub mod inventory_adder {
    use super::*;
    pub fn add(ctx: Context<Add>, id: u32) -> Result<()> {
        // 脆弱：任意アカウントへ書き込み
        let buf = &mut ctx.accounts.inventory_info.try_borrow_mut_data()?;
        if buf.len() >= 4 {
            let bytes = id.to_le_bytes();
            buf[0] = bytes[0];
            buf[1] = bytes[1];
            buf[2] = bytes[2];
            buf[3] = bytes[3];
        }
        msg!("管理者 {} が追加", ctx.accounts.manager.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Add<'info> {
    /// CHECK: 脆弱アカウント（検証なし）
    pub inventory_info: AccountInfo<'info>,
    #[account(has_one = manager)]
    pub inv_admin: Account<'info, InventoryAdmin>,
    pub manager: Signer<'info>,
}

#[account]
pub struct InventoryAdmin {
    pub manager: Pubkey,
}
