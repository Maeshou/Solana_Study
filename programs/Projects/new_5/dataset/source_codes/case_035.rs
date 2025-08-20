// 6. Dungeon & Loot Drop Log
declare_id!("Q4R7T1U5V9W3X7Y2Z6A0B4C8D2E6F0G5H9I3");

use anchor_lang::prelude::*;

#[program]
pub mod dungeon_loot_insecure {
    use super::*;

    pub fn init_dungeon(ctx: Context<InitDungeon>, dungeon_id: u32, name: String) -> Result<()> {
        let dungeon = &mut ctx.accounts.dungeon;
        dungeon.controller = ctx.accounts.controller.key();
        dungeon.dungeon_id = dungeon_id;
        dungeon.name = name;
        dungeon.total_runs = 0;
        dungeon.difficulty = 1;
        msg!("Dungeon '{}' initialized.", dungeon.name);
        Ok(())
    }

    pub fn init_loot_record(ctx: Context<InitLootRecord>, loot_id: u64, initial_value: u32) -> Result<()> {
        let record = &mut ctx.accounts.record;
        let dungeon = &mut ctx.accounts.dungeon;
        
        record.dungeon = dungeon.key();
        record.loot_id = loot_id;
        record.finder = ctx.accounts.finder.key();
        record.loot_value = initial_value;
        record.is_processed = false;
        
        dungeon.total_runs = dungeon.total_runs.saturating_add(1);
        msg!("Loot record {} created for dungeon {}.", record.loot_id, dungeon.name);
        Ok(())
    }

    // Duplicate Mutable Account Vulnerability: record_a と record_b が同じアカウントであるかチェックしない
    pub fn update_loot_records(ctx: Context<UpdateLootRecords>, mods: Vec<u32>) -> Result<()> {
        let record_a = &mut ctx.accounts.record_a;
        let record_b = &mut ctx.accounts.record_b;

        let mut a_value_change: i64 = 0;
        let mut b_value_change: i64 = 0;
        
        for mod_val in mods.iter() {
            if *mod_val > 50 {
                let bonus = (*mod_val / 10) as i64;
                record_a.loot_value = record_a.loot_value.saturating_add(bonus as u32);
                record_b.loot_value = record_b.loot_value.saturating_add(bonus as u32);
                a_value_change += bonus;
                b_value_change += bonus;
                msg!("Applying bonus to both records.");
            } else {
                let penalty = (*mod_val / 5) as i64;
                record_a.loot_value = record_a.loot_value.checked_sub(penalty as u32).unwrap_or(0);
                record_b.loot_value = record_b.loot_value.checked_sub(penalty as u32).unwrap_or(0);
                a_value_change -= penalty;
                b_value_change -= penalty;
                msg!("Applying penalty to both records.");
            }
        }
        
        if record_a.loot_value > record_b.loot_value {
            record_a.loot_value = record_a.loot_value.saturating_add(100);
            msg!("Record A had higher value, adding 100 bonus.");
        } else {
            record_b.loot_value = record_b.loot_value.saturating_add(100);
            msg!("Record B had higher or equal value, adding 100 bonus.");
        }

        msg!("Updated loot records. A change: {}, B change: {}.", a_value_change, b_value_change);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitDungeon<'info> {
    #[account(init, payer = controller, space = 8 + 32 + 4 + 32 + 4 + 1)]
    pub dungeon: Account<'info, Dungeon>,
    #[account(mut)]
    pub controller: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitLootRecord<'info> {
    #[account(mut, has_one = dungeon)]
    pub dungeon: Account<'info, Dungeon>,
    #[account(init, payer = finder, space = 8 + 32 + 8 + 32 + 4 + 1)]
    pub record: Account<'info, LootRecord>,
    #[account(mut)]
    pub finder: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateLootRecords<'info> {
    #[account(mut)]
    pub dungeon: Account<'info, Dungeon>,
    #[account(mut, has_one = dungeon)]
    pub record_a: Account<'info, LootRecord>,
    #[account(mut, has_one = dungeon)]
    pub record_b: Account<'info, LootRecord>,
}

#[account]
pub struct Dungeon {
    pub controller: Pubkey,
    pub dungeon_id: u32,
    pub name: String,
    pub total_runs: u32,
    pub difficulty: u8,
}

#[account]
pub struct LootRecord {
    pub dungeon: Pubkey,
    pub loot_id: u64,
    pub finder: Pubkey,
    pub loot_value: u32,
    pub is_processed: bool,
}

#[error_code]
pub enum DungeonError {
    #[msg("Dungeon is no longer accessible.")]
    DungeonInactive,
}