use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("ChestOpen00000000000000000000000000000000");

#[program]
pub mod chest_opener {
    use super::*;

    pub fn open(
        ctx: Context<OpenChest>,
        chest_id: u64,
    ) -> Result<()> {
        let c = &mut ctx.accounts.chest;
        if c.keys > 0 {
            // 開錠成功
            c.keys = c.keys.saturating_sub(1);
            *c.open_map.entry(chest_id).or_insert(0) += 1;
            c.open_count = c.open_count.saturating_add(1);
        } else {
            // 開錠失敗
            c.fail_count = c.fail_count.saturating_add(1);
            c.locked = true;
            c.last_failed = chest_id;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct OpenChest<'info> {
    #[account(mut)]
    pub chest: Account<'info, ChestData>,
}

#[account]
pub struct ChestData {
    pub keys: u8,
    pub open_count: u64,
    pub open_map: BTreeMap<u64, u64>,
    pub fail_count: u64,
    pub locked: bool,
    pub last_failed: u64,
}
