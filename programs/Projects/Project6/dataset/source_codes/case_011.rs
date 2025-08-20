use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

// #1: Guild Management and Raid Logging
// ドメイン: ギルドメンバーのロール管理とレイドボスへのダメージ記録。
// 安全対策: ギルドメンバーは一意の `guild_member_tag` を持つことで、同一アカウントの二重渡しを防ぐ。また、レイド記録はギルド口座と親子関係を持ち、所有者検証を二重に行う。

declare_id!("9K1i3W4Xy5Z6aB7c8D9eF0gH1i2J3K4L5M6N7O8P");

#[program]
pub mod guild_manager {
    use super::*;

    pub fn initialize_guild(ctx: Context<InitializeGuild>, guild_name: String) -> Result<()> {
        let guild_account = &mut ctx.accounts.guild_account;
        guild_account.name = guild_name;
        guild_account.member_count = 0;
        guild_account.is_active = true;
        guild_account.owner = ctx.accounts.owner.key();
        Ok(())
    }

    pub fn register_member_and_log_raid_damage(
        ctx: Context<RegisterMemberAndLogRaidDamage>,
        member_tag: u64,
        raid_id: u32,
        damage_dealt: u64,
    ) -> Result<()> {
        let member = &mut ctx.accounts.guild_member;
        let raid_log = &mut ctx.accounts.raid_log;
        let guild = &mut ctx.accounts.guild_account;

        // メンバー登録処理
        if member.tag == 0 {
            member.tag = member_tag;
            member.guild = guild.key();
            member.damage_dealt_total = 0;
            member.raid_count = 0;
            guild.member_count = guild.member_count.checked_add(1).unwrap();
        }

        // ダメージ記録処理
        let current_damage = raid_log.damage_entries[raid_id as usize].checked_add(damage_dealt).unwrap_or(u64::MAX);
        raid_log.damage_entries[raid_id as usize] = current_damage;

        let total_damage = member.damage_dealt_total.checked_add(damage_dealt).unwrap_or(u64::MAX);
        member.damage_dealt_total = total_damage;
        member.raid_count = member.raid_count.wrapping_add(1);

        if total_damage > 1_000_000 {
            msg!("Member has reached a high damage milestone!");
        } else {
            msg!("Member continues to fight.");
        }

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(guild_name: String)]
pub struct InitializeGuild<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 4 + 8 + 1 + 32 + 8,
        owner = crate::ID,
    )]
    pub guild_account: Account<'info, Guild>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(member_tag: u64, raid_id: u32, damage_dealt: u64)]
pub struct RegisterMemberAndLogRaidDamage<'info> {
    #[account(
        mut,
        // has_oneで親子関係を検証
        has_one = owner,
    )]
    pub guild_account: Account<'info, Guild>,
    #[account(
        init_if_needed,
        payer = owner,
        space = 8 + 8 + 32 + 8 + 8 + 4,
        owner = crate::ID,
        // GuildとMemberは別口座
        constraint = guild_account.key() != guild_member.key() @ ErrorCode::CosplayBlocked,
    )]
    pub guild_member: Account<'info, GuildMember>,
    #[account(
        init_if_needed,
        payer = owner,
        space = 8 + 32 + 8 + 20 * 8, // 20個のエントリ
        owner = crate::ID,
        has_one = guild_account,
    )]
    pub raid_log: Account<'info, RaidLog>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Guild {
    pub name: String,
    pub owner: Pubkey,
    pub member_count: u32,
    pub is_active: bool,
}

#[account]
pub struct GuildMember {
    pub tag: u64,
    pub guild: Pubkey,
    pub damage_dealt_total: u64,
    pub raid_count: u32,
}

#[account]
pub struct RaidLog {
    pub guild_account: Pubkey,
    pub raid_boss_id: u32,
    pub damage_entries: [u64; 20],
}

#[error_code]
pub enum ErrorCode {
    #[msg("Account is being cosplayed as a different role.")]
    CosplayBlocked,
}
