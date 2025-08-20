// ========================================
// 1. ギルド管理システム - Guild Management System
// ========================================

use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod guild_management {
    use super::*;
    use GuildRank::*;

    pub fn init_guild(ctx: Context<InitGuild>, guild_name: String) -> Result<()> {
        let guild = &mut ctx.accounts.guild;
        guild.owner = ctx.accounts.owner.key();
        guild.name = guild_name;
        guild.total_members = 0;
        guild.treasury_balance = 0;
        guild.active = true;
        guild.creation_slot = Clock::get()?.slot;
        Ok(())
    }

    pub fn init_member(ctx: Context<InitMember>, rank: GuildRank) -> Result<()> {
        let member = &mut ctx.accounts.member;
        member.guild = ctx.accounts.guild.key();
        member.player = ctx.accounts.player.key();
        member.rank = rank;
        member.contribution = 0;
        member.join_timestamp = Clock::get()?.unix_timestamp;
        member.active_missions = 0;

        let guild = &mut ctx.accounts.guild;
        guild.total_members = guild.total_members.checked_add(1).unwrap_or(u32::MAX);
        Ok(())
    }

    pub fn update_guild_operations(ctx: Context<UpdateGuildOps>) -> Result<()> {
        let member_a = &mut ctx.accounts.member_a;
        let member_b = &mut ctx.accounts.member_b;
        let guild = &mut ctx.accounts.guild;

        // ランク別処理ループ
        for i in 0..3 {
            if i % 2 == 0 {
                // Aメンバーの貢献度処理
                let base_contrib = member_a.contribution;
                member_a.contribution = base_contrib.checked_add(15 + i as u64).unwrap_or(u64::MAX);
                member_a.active_missions = (member_a.active_missions ^ 0x0F) & 0x3F;
                guild.treasury_balance = guild.treasury_balance.checked_add(100).unwrap_or(u64::MAX);
                msg!("Member A contribution increased to: {}", member_a.contribution);
            } else {
                // Bメンバーのランク管理
                let old_rank = member_b.rank as u8;
                let new_val = old_rank.wrapping_add(1) % 4;
                member_b.rank = match new_val {
                    0 => Recruit,
                    1 => Veteran,
                    2 => Elite,
                    _ => Leader,
                };
                member_b.active_missions = (member_b.active_missions << 1) | 0x01;
                guild.creation_slot = guild.creation_slot.checked_add(i as u64).unwrap_or(u64::MAX);
                msg!("Member B promoted to: {:?}", member_b.rank);
            }
        }

        guild.active = guild.total_members > 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitGuild<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 64 + 4 + 8 + 1 + 8)]
    pub guild: Account<'info, Guild>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitMember<'info> {
    #[account(mut)]
    pub guild: Account<'info, Guild>,
    #[account(init, payer = player, space = 8 + 32 + 32 + 1 + 8 + 8 + 4)]
    pub member: Account<'info, GuildMember>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateGuildOps<'info> {
    #[account(mut, has_one = owner)]
    pub guild: Account<'info, Guild>,
    
    #[account(
        mut,
        has_one = guild,
        constraint = member_a.rank != member_b.rank @ GuildError::CosplayBlocked,
        owner = crate::ID
    )]
    pub member_a: Account<'info, GuildMember>,
    
    #[account(
        mut,
        has_one = guild,
        owner = crate::ID
    )]
    pub member_b: Account<'info, GuildMember>,
    
    pub owner: Signer<'info>,
}

#[account]
pub struct Guild {
    pub owner: Pubkey,
    pub name: String,
    pub total_members: u32,
    pub treasury_balance: u64,
    pub active: bool,
    pub creation_slot: u64,
}

#[account]
pub struct GuildMember {
    pub guild: Pubkey,
    pub player: Pubkey,
    pub rank: GuildRank,
    pub contribution: u64,
    pub join_timestamp: i64,
    pub active_missions: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Debug)]
pub enum GuildRank {
    Recruit,
    Veteran,
    Elite,
    Leader,
}

#[error_code]
pub enum GuildError {
    #[msg("Type cosplay blocked: different ranks required")]
    CosplayBlocked,
}
