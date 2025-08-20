// 4. 在庫追加（IDの範囲で異なる書き込み）
use anchor_lang::prelude::*;

#[program]
pub mod inventory_adder {
    use super::*;
    pub fn add(ctx: Context<Add>, item_id: u32) -> Result<()> {
        let buf = &mut ctx.accounts.inventory_info.try_borrow_mut_data()?;
        if item_id < 100 {
            // 小さいIDなら先頭4バイトに書き込み
            buf[..4].copy_from_slice(&item_id.to_le_bytes());
        } else {
            // 大きいIDなら末尾4バイトに書き込み
            let end = buf.len();
            buf[end-4..end].copy_from_slice(&item_id.to_le_bytes());
        }
        msg!("在庫管理者 {} が追加実行", ctx.accounts.manager.key());
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
pub struct InventoryAdmin { pub manager: Pubkey }
