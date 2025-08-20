use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("ForgeItem2222222222222222222222222222222222");

#[program]
pub mod forge_item {
    use super::*;

    pub fn forge(ctx: Context<Forge>, material_id: u64, required: u32) -> Result<()> {
        let inv = &mut ctx.accounts.inventory;
        if let Some(&qty) = inv.materials.get(&material_id) {
            if qty >= required {
                // 正常合成
                inv.materials.insert(material_id, qty - required);
                let count = inv.items.entry(material_id).or_insert(0);
                *count = count.saturating_add(1);
                inv.forge_count = inv.forge_count.saturating_add(1);
            } else {
                // 合成失敗
                inv.failure_count = inv.failure_count.saturating_add(1);
                let loss = required / 2;
                let new_qty = qty.saturating_sub(loss);
                inv.materials.insert(material_id, new_qty);
                inv.fail_history
                    .entry(material_id)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            }
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Forge<'info> {
    #[account(mut)]
    pub inventory: Account<'info, ForgeData>,
}

#[account]
pub struct ForgeData {
    pub materials: BTreeMap<u64, u32>,
    pub items: BTreeMap<u64, u32>,
    pub forge_count: u64,
    pub failure_count: u64,
    pub fail_history: BTreeMap<u64, u64>,
}
