// ==================== 1. ギルド管理システム ====================
// ギルドメンバーの階層管理とレイド参加ログを安全に記録するシステム

use anchor_lang::prelude::*;

declare_id!("A1B2C3D4E5F6G7H8I9J0K1L2M3N4O5P6Q7R8S9T0");

#[program]
pub mod guild_management {
    use super::*;
    
    pub fn init_guild(
        ctx: Context<InitGuild>,
        guild_name: String,
        max_members: u32,
    ) -> Result<()> {
        let guild = &mut ctx.accounts.guild;
        guild.owner = ctx.accounts.owner.key();
        guild.guild_name = guild_name;
        guild.max_members = max_members;
        guild.member_count = 0;
        guild.total_raids = 0;
        guild.is_recruiting = true;
        guild.creation_timestamp = Clock::get()?.unix_timestamp;
        
        msg!("Guild initialized: {}", guild.guild_name);
        Ok(())
    }
    
    pub fn init_member(
        ctx: Context<InitMember>,
        member_name: String,
        member_tier: MemberTier,
    ) -> Result<()> {
        let member = &mut ctx.accounts.member;
        member.guild = ctx.accounts.guild.key();
        member.member_name = member_name;
        member.tier = member_tier;
        member.experience_points = 0;
        member.join_timestamp = Clock::get()?.unix_timestamp;
        member.raids_completed = 0;
        member.is_active = true;
        
        msg!("Member {} joined with tier {:?}", member.member_name, member.tier);
        Ok(())
    }
    
    pub fn process_raid_activities(
        ctx: Context<ProcessRaidActivities>,
        raid_difficulty: u8,
        bonus_multiplier: u32,
    ) -> Result<()> {
        let guild = &mut ctx.accounts.guild;
        let member_a = &mut ctx.accounts.member_a;
        let member_b = &mut ctx.accounts.member_b;
        let raid_log = &mut ctx.accounts.raid_log;
        
        // レイド処理のループ
        for round in 0..raid_difficulty {
            if member_a.tier == MemberTier::Leader {
                // リーダー特典処理
                member_a.experience_points = member_a.experience_points
                    .checked_add((round as u64) * 150)
                    .unwrap_or(u64::MAX);
                member_a.raids_completed = member_a.raids_completed
                    .checked_add(2)
                    .unwrap_or(u32::MAX);
                guild.total_raids = guild.total_raids
                    .checked_add(1)
                    .unwrap_or(u64::MAX);
                msg!("Leader {} gained double experience", member_a.member_name);
            } else {
                // 通常メンバー処理
                member_a.experience_points = member_a.experience_points
                    .checked_add((round as u64) * 75)
                    .unwrap_or(u64::MAX);
                member_a.raids_completed = member_a.raids_completed
                    .checked_add(1)
                    .unwrap_or(u32::MAX);
                guild.total_raids = guild.total_raids
                    .checked_add(1)
                    .unwrap_or(u64::MAX);
                msg!("Member {} gained standard experience", member_a.member_name);
            }
        }
        
        // メンバーBも同様の処理
        let mut b_multiplier = 1;
        while b_multiplier <= bonus_multiplier {
            member_b.experience_points = member_b.experience_points
                .checked_add(b_multiplier as u64 * 25)
                .unwrap_or(u64::MAX);
            
            // ビット操作でボーナス計算
            let bit_bonus = (b_multiplier ^ 0x3) << 2;
            member_b.raids_completed = member_b.raids_completed
                .checked_add(bit_bonus)
                .unwrap_or(u32::MAX);
            
            b_multiplier = b_multiplier.wrapping_mul(2);
        }
        
        // レイドログ更新
        raid_log.raid_count = raid_log.raid_count
            .checked_add(1)
            .unwrap_or(u64::MAX);
        raid_log.last_raid_timestamp = Clock::get()?.unix_timestamp;
        raid_log.difficulty_sum = raid_log.difficulty_sum
            .checked_add(raid_difficulty as u32)
            .unwrap_or(u32::MAX);
        
        msg!("Raid activities processed for {} rounds", raid_difficulty);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitGuild<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 64 + 4 + 4 + 8 + 1 + 8
    )]
    pub guild: Account<'info, Guild>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitMember<'info> {
    #[account(mut)]
    pub guild: Account<'info, Guild>,
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 64 + 1 + 8 + 8 + 4 + 1
    )]
    pub member: Account<'info, GuildMember>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessRaidActivities<'info> {
    #[account(mut, has_one = owner)]
    pub guild: Account<'info, Guild>,
    #[account(
        mut,
        has_one = guild,
        owner = crate::ID,
        constraint = member_a.tier != member_b.tier @ GuildErrorCode::CosplayBlocked
    )]
    pub member_a: Account<'info, GuildMember>,
    #[account(
        mut,
        has_one = guild,
        owner = crate::ID,
        constraint = member_a.member_name != member_b.member_name @ GuildErrorCode::CosplayBlocked
    )]
    pub member_b: Account<'info, GuildMember>,
    #[account(
        init_if_needed,
        payer = owner,
        space = 8 + 32 + 8 + 8 + 4,
        seeds = [b"raid_log", guild.key().as_ref()],
        bump
    )]
    pub raid_log: Account<'info, RaidLog>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Guild {
    pub owner: Pubkey,
    pub guild_name: String,
    pub max_members: u32,
    pub member_count: u32,
    pub total_raids: u64,
    pub is_recruiting: bool,
    pub creation_timestamp: i64,
}

#[account]
pub struct GuildMember {
    pub guild: Pubkey,
    pub member_name: String,
    pub tier: MemberTier,
    pub experience_points: u64,
    pub join_timestamp: i64,
    pub raids_completed: u32,
    pub is_active: bool,
}

#[account]
pub struct RaidLog {
    pub guild: Pubkey,
    pub raid_count: u64,
    pub last_raid_timestamp: i64,
    pub difficulty_sum: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum MemberTier {
    Recruit,
    Member,
    Veteran,
    Officer,
    Leader,
}

use MemberTier::*;

#[error_code]
pub enum GuildErrorCode {
    #[msg("Type cosplay detected: Different accounts cannot have same identifier")]
    CosplayBlocked,
    #[msg("Guild is full")]
    GuildFull,
    #[msg("Insufficient permissions")]
    InsufficientPermissions,
}