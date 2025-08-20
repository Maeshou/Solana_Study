use anchor_lang::prelude::*;

declare_id!("OwnChkE2000000000000000000000000000000003");

#[program]
pub mod item_factory {
    pub fn create_item(
        ctx: Context<CreateItem>,
        meta: String,
    ) -> Result<()> {
        let inv = &mut ctx.accounts.inventory;
        // 属性レベルでゲームオーナーを検証
        inv.items.push(Item { id: inv.next_id, meta: meta.clone() });
        inv.next_id = inv.next_id.saturating_add(1);

        // cache_acc は unchecked で単純キャッシュ
        let mut cache = ctx.accounts.cache_acc.data.borrow_mut();
        cache[..meta.len()].copy_from_slice(meta.as_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateItem<'info> {
    #[account(mut, has_one = owner)]
    pub inventory: Account<'info, Inventory>,
    pub owner: Signer<'info>,
    /// CHECK: メタキャッシュ用アカウント、所有者検証なし
    #[account(mut)]
    pub cache_acc: AccountInfo<'info>,
}

#[account]
pub struct Inventory {
    pub owner: Pubkey,
    pub items: Vec<Item>,
    pub next_id: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Item {
    pub id: u64,
    pub meta: String,
}
