use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

declare_id!("NFTGameVuln9999999999999999999999999999999");

#[program]
pub mod battle_arena_revival_demo {
    use super::*;

    pub fn conclude_battle_arena(ctx: Context<ConcludeBattleArena>) -> Result<()> {
        // バトルアリーナを終了して戦利品を分配
        Ok(())
    }

    pub fn reopen_arena_same_tx(
        ctx: Context<ReopenArenaSameTx>,
        arena_size: u64,
        difficulty_level: u8,
    ) -> Result<()> {
        let arena_account = ctx.accounts.battle_arena_addr.to_account_info();
        let arena_master = ctx.accounts.arena_master.to_account_info();

        let difficulty_cost_multiplier = (difficulty_level as u64).pow(2);
        let base_arena_cost = 900_000;
        let enhanced_cost = base_arena_cost + (difficulty_cost_multiplier * 100_000);

        let reopen_arena_funding = system_instruction::transfer(
            &arena_master.key(),
            &arena_account.key(),
            enhanced_cost
        );
        anchor_lang::solana_program::program::invoke(
            &reopen_arena_funding,
            &[arena_master.clone(), arena_account.clone()],
        )?;

        let allocate_arena_space = system_instruction::allocate(&arena_account.key(), arena_size);
        anchor_lang::solana_program::program::invoke(
            &allocate_arena_space,
            &[arena_account.clone()]
        )?;

        let claim_arena_control = system_instruction::assign(&arena_account.key(), &crate::id());
        anchor_lang::solana_program::program::invoke(
            &claim_arena_control,
            &[arena_account.clone()]
        )?;

        let mut arena_data = arena_account.try_borrow_mut_data()?;
        arena_data[0] = difficulty_level;
        
        let size_bytes = (arena_size as u32).to_be_bytes();
        for (index, size_byte) in size_bytes.iter().enumerate() {
            arena_data[1 + index] = *size_byte;
        }
        
        arena_data[5] = 100u8;
        arena_data[6] = 200u8;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ConcludeBattleArena<'info> {
    #[account(mut, close = victory_vault)]
    pub battle_arena: Account<'info, BattleArenaData>,
    #[account(mut)]
    pub victory_vault: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct ReopenArenaSameTx<'info> {
    #[account(mut)]
    pub battle_arena_addr: UncheckedAccount<'info>,
    #[account(mut)]
    pub arena_master: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct BattleArenaData {
    pub max_participants: u16,
    pub current_round: u8,
}