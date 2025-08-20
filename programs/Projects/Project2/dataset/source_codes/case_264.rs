use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("ForgeStat033333333333333333333333333333333");

#[program]
pub mod forge_station {
    use super::*;

    pub fn forge_item(ctx: Context<Forge>, material_id: u64) -> Result<()> {
        let fs = &mut ctx.accounts.station;
        // 耐久消費
        if fs.durability > 0 {
            fs.durability = fs.durability.saturating_sub(1);
            *fs.products.entry(material_id).or_insert(0) += 1;
        } else {
            // 耐久ゼロならスクラップ蓄積
            *fs.scrap.entry(material_id).or_insert(0) += 1;
            fs.scrap_count = fs.scrap_count.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Forge<'info> {
    #[account(mut)]
    pub station: Account<'info, ForgeStationData>,
}

#[account]
pub struct ForgeStationData {
    pub durability: u8,
    pub products: BTreeMap<u64, u64>,
    pub scrap: BTreeMap<u64, u64>,
    pub scrap_count: u64,
}
