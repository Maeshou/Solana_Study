// 8. コレクション管理＋追加／クリア
use anchor_lang::prelude::*;

declare_id!("Col88888888888888888888888888888888");

#[program]
pub mod reinit_collection_v2 {
    use super::*;

    // 初期アイテムをセット
    pub fn setup_collection(
        ctx: Context<SetupCollection>,
        items: Vec<Pubkey>,
    ) -> Result<()> {
        let col = &mut ctx.accounts.collection;
        col.items = items.clone();
        col.size = items.len() as u32;
        Ok(())
    }

    // アイテムを追加
    pub fn add_item(
        ctx: Context<SetupCollection>,
        item: Pubkey,
    ) -> Result<()> {
        let col = &mut ctx.accounts.collection;
        col.items.push(item);
        col.size = col.items.len() as u32;
        Ok(())
    }

    // 全アイテムをクリア
    pub fn clear_items(
        ctx: Context<SetupCollection>,
    ) -> Result<()> {
        let col = &mut ctx.accounts.collection;
        col.items.clear();
        col.size = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetupCollection<'info> {
    #[account(mut)]
    pub collection: Account<'info, CollectionData>,
    /// CHECK: クリエイター用、処理なし
    #[account(mut)]
    pub owner_meta: AccountInfo<'info>,
}

#[account]
pub struct CollectionData {
    pub items: Vec<Pubkey>,
    pub size: u32,
}
