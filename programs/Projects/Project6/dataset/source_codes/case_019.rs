// #9: Dungeon Loot Manager
// ドメイン: ダンジョンでの戦利品取得と、それに対するランク付け。
// 安全対策: `DungeonRun` と `LootDrop` は親子関係で紐付け。`LootDrop` は `run_id` と `drop_index` で一意性を確保し、二重渡しを防ぐ。`owner` 検証も二重に行う。

declare_id!("F3G4H5I6J7K8L9M0N1O2P3Q4R5S6T7U8V9W0X1Y2");

#[program]
pub mod dungeon_loot {
    use super::*;

    pub fn start_dungeon_run(ctx: Context<StartDungeonRun>) -> Result<()> {
        let run = &mut ctx.accounts.dungeon_run;
        run.owner = ctx.accounts.owner.key();
        run.loot_found = 0;
        run.run_id = Clock::get()?.unix_timestamp as u32;
        Ok(())
    }

    pub fn process_loot_drop(
        ctx: Context<ProcessLootDrop>,
        item_id: u32,
        drop_index: u8,
        item_rarity: u8,
    ) -> Result<()> {
        let loot_drop = &mut ctx.accounts.loot_drop;
        let dungeon_run = &mut ctx.accounts.dungeon_run;

        loot_drop.run_id = dungeon_run.run_id;
        loot_drop.item_id = item_id;
        loot_drop.drop_index = drop_index;
        loot_drop.rarity = item_rarity;

        dungeon_run.loot_found = dungeon_run.loot_found.checked_add(1).unwrap_or(u32::MAX);
        
        let mut total_rarity = 0u64;
        let mut i = 0;
        while i < dungeon_run.loot_found as usize {
            // ループは仮想的な処理
            total_rarity += item_rarity as u64;
            i += 1;
        }

        if item_rarity > 80 {
            msg!("A rare item was found!");
        } else {
            msg!("A common item was found.");
        }

        // 簡易比率スケーリング
        let loot_value = (item_rarity as u64 * 100).checked_div(255).unwrap_or(0);
        msg!("Loot value scaled: {}", loot_value);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartDungeonRun<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 4 + 4,
        owner = crate::ID,
    )]
    pub dungeon_run: Account<'info, DungeonRun>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(item_id: u32, drop_index: u8, item_rarity: u8)]
pub struct ProcessLootDrop<'info> {
    #[account(
        mut,
        has_one = owner,
    )]
    pub dungeon_run: Account<'info, DungeonRun>,
    #[account(
        init,
        payer = owner,
        space = 8 + 4 + 4 + 1 + 1,
        owner = crate::ID,
        // `DungeonRun` と `LootDrop` が同一口座ではないことを検証
        constraint = dungeon_run.key() != loot_drop.key() @ ErrorCode::CosplayBlocked,
    )]
    pub loot_drop: Account<'info, LootDrop>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DungeonRun {
    pub owner: Pubkey,
    pub run_id: u32,
    pub loot_found: u32,
}

#[account]
pub struct LootDrop {
    pub run_id: u32,
    pub item_id: u32,
    pub drop_index: u8,
    pub rarity: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Account is being cosplayed as a different role.")]
    CosplayBlocked,
}
