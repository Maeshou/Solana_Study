// 2. プログラム名: DungeonDelvers
use anchor_lang::prelude::*;

declare_id!("R9zD8cK1T4L2H5M7N6P3Q8S5W2V9U7X4Y1Z0B9A");

#[program]
pub mod dungeon_delvers {
    use super::*;

    pub fn init_dungeon(ctx: Context<InitDungeon>, level_count: u32, base_difficulty: u8) -> Result<()> {
        let dungeon = &mut ctx.accounts.dungeon_data;
        dungeon.level_count = level_count.checked_add(10).unwrap_or(u32::MAX);
        dungeon.base_difficulty = base_difficulty.checked_div(2).unwrap_or(1);
        dungeon.total_treasure = 0;
        dungeon.dungeon_status = DungeonStatus::Unexplored;
        msg!("Dungeon created with {} levels and base difficulty {}.", dungeon.level_count, dungeon.base_difficulty);
        Ok(())
    }

    pub fn init_adventurer(ctx: Context<InitAdventurer>, player_id: u64, courage_level: u8) -> Result<()> {
        let adventurer = &mut ctx.accounts.adventurer_sheet;
        adventurer.parent_dungeon = ctx.accounts.dungeon_data.key();
        adventurer.player_id = player_id.checked_add(500).unwrap_or(u64::MAX);
        adventurer.courage_level = courage_level.checked_add(10).unwrap_or(u8::MAX);
        adventurer.is_alive = true;
        adventurer.current_level = 0;
        adventurer.treasure_found = 0;
        msg!("Adventurer {} enters the dungeon with courage level {}.", adventurer.player_id, adventurer.courage_level);
        Ok(())
    }

    pub fn raid_dungeon(ctx: Context<RaidDungeon>, raid_power: u32) -> Result<()> {
        let dungeon = &mut ctx.accounts.dungeon_data;
        let explorer = &mut ctx.accounts.explorer_sheet;
        let scout = &mut ctx.accounts.scout_sheet;
        let mut treasure_gained_total = 0u64;

        while dungeon.level_count > scout.current_level {
            if explorer.is_alive {
                let treasure_amount = (raid_power as u64).checked_add(scout.current_level as u64).unwrap_or(u64::MAX);
                explorer.treasure_found = explorer.treasure_found.checked_add(treasure_amount).unwrap_or(u64::MAX);
                explorer.courage_level = explorer.courage_level.checked_add(dungeon.base_difficulty).unwrap_or(u8::MAX);
                dungeon.total_treasure = dungeon.total_treasure.checked_add(treasure_amount).unwrap_or(u64::MAX);
                treasure_gained_total = treasure_gained_total.checked_add(treasure_amount).unwrap_or(u64::MAX);
                msg!("Explorer found {} treasure on level {}.", treasure_amount, scout.current_level);
            } else {
                let revival_cost = (raid_power as u64).checked_add(dungeon.base_difficulty as u64).unwrap_or(u64::MAX);
                dungeon.total_treasure = dungeon.total_treasure.checked_sub(revival_cost).unwrap_or(0);
                explorer.is_alive = true;
                explorer.treasure_found = explorer.treasure_found.checked_div(2).unwrap_or(0);
                explorer.courage_level = explorer.courage_level.checked_add(10).unwrap_or(u8::MAX);
                msg!("Explorer revived, losing {} treasure.", revival_cost);
            }

            if scout.is_alive {
                let treasure_amount = (raid_power as u64).checked_mul(dungeon.base_difficulty as u64).unwrap_or(u64::MAX);
                scout.treasure_found = scout.treasure_found.checked_add(treasure_amount).unwrap_or(u64::MAX);
                scout.current_level = scout.current_level.checked_add(1).unwrap_or(u32::MAX);
                dungeon.total_treasure = dungeon.total_treasure.checked_add(treasure_amount).unwrap_or(u64::MAX);
                treasure_gained_total = treasure_gained_total.checked_add(treasure_amount).unwrap_or(u64::MAX);
                msg!("Scout advanced to level {} and found {} treasure.", scout.current_level, treasure_amount);
            } else {
                let revival_cost = (raid_power as u64).checked_mul(dungeon.base_difficulty as u64).unwrap_or(u64::MAX);
                dungeon.total_treasure = dungeon.total_treasure.checked_sub(revival_cost).unwrap_or(0);
                scout.is_alive = true;
                scout.treasure_found = scout.treasure_found.checked_div(3).unwrap_or(0);
                scout.courage_level = scout.courage_level.checked_add(20).unwrap_or(u8::MAX);
                msg!("Scout revived, losing {} treasure.", revival_cost);
            }
        }
        msg!("Total treasure gained in this raid: {}", treasure_gained_total);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(level_count: u32, base_difficulty: u8)]
pub struct InitDungeon<'info> {
    #[account(init, payer = signer, space = 8 + 4 + 1 + 8 + 4)]
    pub dungeon_data: Account<'info, DungeonData>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(player_id: u64, courage_level: u8)]
pub struct InitAdventurer<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 8 + 1 + 1 + 4 + 8)]
    pub adventurer_sheet: Account<'info, AdventurerSheet>,
    #[account(mut)]
    pub dungeon_data: Account<'info, DungeonData>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(raid_power: u32)]
pub struct RaidDungeon<'info> {
    #[account(mut)]
    pub dungeon_data: Account<'info, DungeonData>,
    #[account(mut, has_one = parent_dungeon)]
    pub explorer_sheet: Account<'info, AdventurerSheet>,
    #[account(mut, has_one = parent_dungeon)]
    pub scout_sheet: Account<'info, AdventurerSheet>,
    pub signer: Signer<'info>,
}

#[account]
pub struct DungeonData {
    level_count: u32,
    base_difficulty: u8,
    total_treasure: u64,
    dungeon_status: DungeonStatus,
}

#[account]
pub struct AdventurerSheet {
    parent_dungeon: Pubkey,
    player_id: u64,
    courage_level: u8,
    is_alive: bool,
    current_level: u32,
    treasure_found: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum DungeonStatus {
    Unexplored,
    Explored,
    Cleared,
}
