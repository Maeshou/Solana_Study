use anchor_lang::prelude::*;

declare_id!("OwnChkEXT00000000000000000000000000000008");

#[program]
pub mod inventory_ext {
    pub fn add_and_reset(
        ctx: Context<InvExt>,
        item_id: u64,
        qty: u32,
    ) -> Result<()> {
        let inv = &mut ctx.accounts.inv;
        // 所有者検証済み
        inv.items.push((item_id, qty));
        inv.total_count = inv.total_count.saturating_add(qty as u64);

        // cache_acc は unchecked で全部 0 に
        ctx.accounts.cache_acc.data.borrow_mut().fill(0);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InvExt<'info> {
    #[account(mut, has_one = owner)]
    pub inv: Account<'info, InventoryExt>,
    pub owner: Signer<'info>,
    /// CHECK: キャッシュアカウント。所有者検証なし
    #[account(mut)]
    pub cache_acc: AccountInfo<'info>,
}

#[account]
pub struct InventoryExt {
    pub owner: Pubkey,
    pub items: Vec<(u64, u32)>,
    pub total_count: u64,
}
