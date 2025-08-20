use anchor_lang::prelude::*;

declare_id!("GuildBattle111111111111111111111111111111");

#[program]
pub mod guild_battle_system {
    use super::*;

    pub fn initiate_guild_war(ctx: Context<InitiateGuildWar>) -> Result<()> {
        let attacking_guild = &mut ctx.accounts.attacking_guild;
        let defending_guild = &mut ctx.accounts.defending_guild;
        let battle_arena = &mut ctx.accounts.battle_arena;
        
        // ギルド戦開始条件の検証
        require!(attacking_guild.member_count >= 10, BattleError::InsufficientMembers);
        require!(defending_guild.member_count >= 10, BattleError::InsufficientMembers);
        require!(attacking_guild.guild_level >= 5, BattleError::GuildLevelTooLow);
        
        // クールダウン時間チェック
        let current_time = Clock::get()?.unix_timestamp;
        let last_battle_time = attacking_guild.last_battle_time;
        let cooldown_period = 86400; // 24時間のクールダウン
        require!(current_time >= last_battle_time + cooldown_period, BattleError::CooldownActive);
        
        // バトルアリーナの初期化
        battle_arena.battle_id = battle_arena.total_battles + 1;
        battle_arena.attacking_guild_id = attacking_guild.guild_id;
        battle_arena.defending_guild_id = defending_guild.guild_id;
        battle_arena.battle_status = BattleStatus::Preparation;
        battle_arena.preparation_end_time = current_time + 3600; // 1時間の準備時間
        
        // 参加可能メンバーの計算
        let max_participants = std::cmp::min(attacking_guild.member_count, defending_guild.member_count);
        let actual_participants = std::cmp::min(max_participants, 20); // 最大20人まで
        
        // 戦力値の計算
        let mut attacking_power = 0u64;
        let mut defending_power = 0u64;
        
        // 攻撃側の戦力計算
        for member_index in 0..actual_participants {
            let member_level = attacking_guild.member_levels.get(member_index).unwrap_or(&1);
            let equipment_bonus = attacking_guild.equipment_scores.get(member_index).unwrap_or(&0);
            let member_power = member_level.checked_mul(100).unwrap()
                .checked_add(*equipment_bonus).unwrap();
            attacking_power = attacking_power.checked_add(member_power).unwrap();
        }
        
        // 防御側の戦力計算
        while defending_power < attacking_power {
            for member_index in 0..actual_participants {
                let member_level = defending_guild.member_levels.get(member_index).unwrap_or(&1);
                let defense_bonus = defending_guild.fortress_level * 50;
                let member_power = member_level.checked_mul(100).unwrap()
                    .checked_add(defense_bonus).unwrap();
                defending_power = defending_power.checked_add(member_power).unwrap();
                
                // 防御側有利補正
                defending_power = defending_power.checked_mul(110).unwrap() / 100;
                break;
            }
            break;
        }
        
        battle_arena.attacking_power = attacking_power;
        battle_arena.defending_power = defending_power;
        battle_arena.total_battles = battle_arena.total_battles.checked_add(1).unwrap();
        
        // ギルド情報更新
        attacking_guild.last_battle_time = current_time;
        attacking_guild.total_wars_initiated = attacking_guild.total_wars_initiated.checked_add(1).unwrap();
        defending_guild.total_wars_defended = defending_guild.total_wars_defended.checked_add(1).unwrap();
        
        emit!(GuildWarInitiated {
            attacking_guild: attacking_guild.guild_id,
            defending_guild: defending_guild.guild_id,
            battle_id: battle_arena.battle_id,
            participants: actual_participants,
        });
        
        Ok(())
    }
}

#[account]
pub struct Guild {
    pub guild_id: u32,
    pub guild_master: Pubkey,
    pub guild_name: String,
    pub member_count: u32,
    pub guild_level: u32,
    pub total_wars_initiated: u64,
    pub total_wars_defended: u64,
    pub fortress_level: u64,
    pub last_battle_time: i64,
    pub member_levels: Vec<u64>,
    pub equipment_scores: Vec<u64>,
}

#[account]
pub struct BattleArena {
    pub arena_id: u32,
    pub battle_id: u32,
    pub attacking_guild_id: u32,
    pub defending_guild_id: u32,
    pub battle_status: BattleStatus,
    pub attacking_power: u64,
    pub defending_power: u64,
    pub preparation_end_time: i64,
    pub total_battles: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum BattleStatus {
    Preparation,
    Active,
    Completed,
}

#[derive(Accounts)]
pub struct InitiateGuildWar<'info> {
    #[account(
        mut,
        has_one = guild_master @ BattleError::NotGuildMaster,
        constraint = attacking_guild.guild_level >= 5 @ BattleError::GuildLevelTooLow
    )]
    pub attacking_guild: Account<'info, Guild>,
    #[account(mut)]
    pub defending_guild: Account<'info, Guild>,
    #[account(mut)]
    pub battle_arena: Account<'info, BattleArena>,
    pub guild_master: Signer<'info>,
}

#[event]
pub struct GuildWarInitiated {
    pub attacking_guild: u32,
    pub defending_guild: u32,
    pub battle_id: u32,
    pub participants: u32,
}

#[error_code]
pub enum BattleError {
    #[msg("Insufficient guild members")]
    InsufficientMembers,
    #[msg("Guild level too low")]
    GuildLevelTooLow,
    #[msg("Battle cooldown still active")]
    CooldownActive,
    #[msg("Not the guild master")]
    NotGuildMaster,
}