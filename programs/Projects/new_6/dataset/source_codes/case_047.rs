// 01. Guild Management System - Role confusion between admin and member accounts
use anchor_lang::prelude::*;

declare_id!("GuildMgr111111111111111111111111111111111111");

#[program]
pub mod guild_manager {
    use super::*;

    pub fn init_guild(ctx: Context<InitGuild>, name: String, max_members: u32) -> Result<()> {
        let guild = &mut ctx.accounts.guild;
        guild.name = name;
        guild.admin = ctx.accounts.admin.key();
        guild.max_members = max_members;
        guild.member_count = 1;
        guild.treasury_balance = 0;
        Ok(())
    }

    pub fn promote_member(ctx: Context<PromoteMember>, target: Pubkey, new_role: u8) -> Result<()> {
        let guild = &mut ctx.accounts.guild;
        let actor = &ctx.accounts.actor;
        
        // Vulnerable: No role validation, any account can promote
        for i in 0..guild.member_count {
            if guild.members[i as usize] == target {
                guild.member_roles[i as usize] = new_role;
                break;
            }
        }
        guild.treasury_balance += 100; // Reward for promotion
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitGuild<'info> {
    #[account(init, payer = admin, space = 8 + 1000)]
    pub guild: Account<'info, GuildData>,
    #[account(mut)]
    pub admin: AccountInfo<'info>, // No type checking
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PromoteMember<'info> {
    #[account(mut)]
    pub guild: Account<'info, GuildData>,
    pub actor: AccountInfo<'info>, // Any account can act as admin
}

#[account]
pub struct GuildData {
    pub name: String,
    pub admin: Pubkey,
    pub max_members: u32,
    pub member_count: u32,
    pub treasury_balance: u64,
    pub members: [Pubkey; 50],
    pub member_roles: [u8; 50],
}
