// ========== プログラム1: ギルドバトル戦績管理 ==========
// ギルド間の対戦記録とメンバーのランキング管理システム
use anchor_lang::prelude::*;

declare_id!("7A8B9C1D2E3F4G5H6I7J8K9L1M2N3O4P5Q6R7S8T9U1V");

#[program]
pub mod guild_battle_tracker {
    use super::*;
    use GuildTier::*;
    use BattleStatus::*;

    pub fn init_guild_registry(
        ctx: Context<InitGuildRegistry>,
        guild_tag: u32,
        tier: GuildTier,
    ) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        registry.master_guild = ctx.accounts.master.key();
        registry.guild_tag = guild_tag;
        registry.tier = tier;
        registry.total_battles = 1u32;
        registry.win_streak = 1u16;
        Ok(())
    }

    pub fn init_battle_record(
        ctx: Context<InitBattleRecord>,
        battle_id: u64,
        status: BattleStatus,
    ) -> Result<()> {
        let record = &mut ctx.accounts.record;
        record.parent_registry = ctx.accounts.registry.key();
        record.battle_id = battle_id;
        record.status = status;
        record.participant_count = 2u8;
        Ok(())
    }

    pub fn process_battle_results(ctx: Context<ProcessBattleResults>) -> Result<()> {
        let left_reg = &mut ctx.accounts.left_registry;
        let right_reg = &mut ctx.accounts.right_registry;
        let battle_log = &mut ctx.accounts.battle_log;

        for iteration in 1..=5u8 {
            if iteration % 2 == 1 {
                // 奇数回: 左ギルドの処理
                let base_score = left_reg.total_battles.checked_mul(3).unwrap_or(u32::MAX);
                left_reg.total_battles = base_score.checked_add(iteration as u32).unwrap_or(u32::MAX);
                left_reg.win_streak = left_reg.win_streak.saturating_add(1);
                battle_log.victory_points = battle_log.victory_points.wrapping_add(base_score as u64);
                msg!("Left guild processed: iteration {}", iteration);
            } else {
                // 偶数回: 右ギルドの処理
                let penalty = right_reg.win_streak.saturating_sub(1);
                right_reg.win_streak = penalty;
                right_reg.total_battles = right_reg.total_battles.saturating_add(2);
                battle_log.defeat_penalty = battle_log.defeat_penalty.checked_add(penalty as u64).unwrap_or(u64::MAX);
                msg!("Right guild processed: iteration {}", iteration);
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitGuildRegistry<'info> {
    #[account(
        init,
        payer = master,
        space = 8 + 32 + 4 + 1 + 4 + 2
    )]
    pub registry: Account<'info, GuildRegistry>,
    #[account(mut)]
    pub master: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitBattleRecord<'info> {
    #[account(mut, has_one = master_guild)]
    pub registry: Account<'info, GuildRegistry>,
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 8 + 1 + 1
    )]
    pub record: Account<'info, BattleRecord>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessBattleResults<'info> {
    #[account(
        mut,
        has_one = master_guild,
        constraint = left_registry.guild_tag != right_registry.guild_tag @ ErrorCode::CosplayBlocked
    )]
    pub left_registry: Account<'info, GuildRegistry>,
    #[account(
        mut,
        has_one = master_guild,
        owner = crate::ID
    )]
    pub right_registry: Account<'info, GuildRegistry>,
    #[account(mut, has_one = parent_registry)]
    pub battle_log: Account<'info, BattleRecord>,
    pub master_guild: Signer<'info>,
}

#[account]
pub struct GuildRegistry {
    pub master_guild: Pubkey,
    pub guild_tag: u32,
    pub tier: GuildTier,
    pub total_battles: u32,
    pub win_streak: u16,
}

#[account]
pub struct BattleRecord {
    pub parent_registry: Pubkey,
    pub battle_id: u64,
    pub status: BattleStatus,
    pub participant_count: u8,
    pub victory_points: u64,
    pub defeat_penalty: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum GuildTier {
    Bronze,
    Silver,
    Gold,
    Diamond,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum BattleStatus {
    Pending,
    Active,
    Completed,
    Cancelled,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Type cosplay detected - same account used for different roles")]
    CosplayBlocked,
}