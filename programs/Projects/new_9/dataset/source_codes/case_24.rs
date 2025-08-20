use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

declare_id!("NFTGameVuln3333333333333333333333333333333");

#[program]
pub mod dungeon_crawler {
    use super::*;

    // 新しいダンジョンをオープンする（脆弱な再生成処理）
    pub fn reopen_dungeon(ctx: Context<ReopenDungeon>, level: u8, monster_count: u16) -> Result<()> {
        let dungeon_account = ctx.accounts.dungeon_instance.to_account_info();
        let organizer = ctx.accounts.organizer.to_account_info();
        let dungeon_size = (monster_count as u64) * 16 + 32;

        let funding = 2_000_000 + (level as u64) * 100_000;

        let transfer_ix = system_instruction::transfer(&organizer.key(), &dungeon_account.key(), funding);
        anchor_lang::solana_program::program::invoke(
            &transfer_ix,
            &[organizer.clone(), dungeon_account.clone()],
        )?;

        let allocate_ix = system_instruction::allocate(&dungeon_account.key(), dungeon_size);
        anchor_lang::solana_program::program::invoke(&allocate_ix, &[dungeon_account.clone()])?;

        let assign_ix = system_instruction::assign(&dungeon_account.key(), &crate::id());
        anchor_lang::solana_program::program::invoke(&assign_ix, &[dungeon_account.clone()])?;

        let mut data = dungeon_account.try_borrow_mut_data()?;
        data[8] = level;
        data[9..11].copy_from_slice(&monster_count.to_le_bytes());
        data[11] = 100; // Initial boss health percentage

        Ok(())
    }

    // ダンジョンをクリアし、インスタンスをクローズする
    pub fn clear_dungeon(ctx: Context<ClearDungeon>) -> Result<()> {
        msg!("Dungeon cleared! All loot has been claimed.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClearDungeon<'info> {
    #[account(mut, close = victor)]
    pub dungeon_instance: Account<'info, DungeonInstance>,
    #[account(mut)]
    pub victor: Signer<'info>,
}

#[account]
pub struct DungeonInstance {
    pub level: u8,
    pub monsters_remaining: u16,
    pub boss_health: u64,
}

#[derive(Accounts)]
pub struct ReopenDungeon<'info> {
    #[account(mut)]
    pub dungeon_instance: UncheckedAccount<'info>,
    #[account(mut)]
    pub organizer: Signer<'info>,
    pub system_program: Program<'info, System>,
}