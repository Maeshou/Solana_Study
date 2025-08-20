use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("DngeRun4444444444444444444444444444444444");

#[program]
pub mod dungeon_runner {
    use super::*;

    pub fn enter(
        ctx: Context<EnterDungeon>,
        health: u64,
        threshold: u64,
    ) -> Result<()> {
        let d = &mut ctx.accounts.dungeon;
        if health >= threshold {
            // クリア
            d.clear_count = d.clear_count.saturating_add(1);
            let loot = threshold / 10;
            d.loot_map.insert(ctx.accounts.player.key(), loot);
        } else {
            // 失敗
            d.fail_count = d.fail_count.saturating_add(1);
            d.locked = true;
            d.penalty_map
                .entry(ctx.accounts.player.key())
                .and_modify(|c| *c += threshold - health)
                .or_insert(threshold - health);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct EnterDungeon<'info> {
    #[account(mut)]
    pub dungeon: Account<'info, DungeonData>,
    pub player: Signer<'info>,
}

#[account]
pub struct DungeonData {
    pub clear_count: u64,
    pub fail_count: u64,
    pub locked: bool,
    pub loot_map: BTreeMap<Pubkey, u64>,
    pub penalty_map: BTreeMap<Pubkey, u64>,
}
