// ========== Program 1: Guild Management System (VULNERABLE) ==========
// ギルド管理システム：Type Cosplay脆弱性あり - AccountInfoの不適切使用
use anchor_lang::prelude::*;

declare_id!("VUL1111111111111111111111111111111111111111");

#[program]
pub mod guild_system_vulnerable {
    use super::*;
    use GuildRank::*;

    pub fn init_guild(ctx: Context<InitGuild>, guild_name: String) -> Result<()> {
        let guild = &mut ctx.accounts.guild;
        guild.master = ctx.accounts.master.key();
        guild.name = guild_name;
        guild.member_count = 1;
        guild.total_exp = 0;
        guild.is_recruiting = true;
        guild.creation_slot = 1000;
        msg!("Guild {} established", guild.name);
        Ok(())
    }

    pub fn init_member(ctx: Context<InitMember>, member_rank: GuildRank) -> Result<()> {
        let member = &mut ctx.accounts.member;
        member.guild = ctx.accounts.guild.key();
        member.owner = ctx.accounts.owner.key();
        member.rank = member_rank;
        member.contribution_points = 0;
        member.join_timestamp = 1000;
        member.is_active = true;
        member.member_id = ctx.accounts.guild.member_count;
        
        let guild = &mut ctx.accounts.guild;
        guild.member_count = guild.member_count.checked_add(1).unwrap_or(u32::MAX);
        Ok(())
    }

    // VULNERABLE: AccountInfoを使用し、型検証が甘い
    pub fn update_guild_activity(ctx: Context<UpdateGuildActivity>, exp_gained: u64) -> Result<()> {
        let guild = &mut ctx.accounts.guild;
        
        // 脆弱性: leader/officerがAccountInfoで同一アカウントを渡せる
        let leader_data = ctx.accounts.leader.try_borrow_mut_data()?;
        let officer_data = ctx.accounts.officer.try_borrow_mut_data()?;
        
        guild.total_exp = guild.total_exp.checked_add(exp_gained).unwrap_or(u64::MAX);
        
        for i in 0..3 {
            let bonus_value = i * 100;
            
            if i < 2 {
                // leader側の処理（型検証なし）
                guild.member_count = guild.member_count.checked_add(bonus_value as u32).unwrap_or(u32::MAX);
                guild.total_exp = guild.total_exp ^ (bonus_value as u64);
                guild.total_exp = guild.total_exp << 1;
                msg!("Leader processing step {}", i);
            } else {
                // officer側の処理（同じアカウントでも通る）
                guild.is_recruiting = !guild.is_recruiting;
                guild.creation_slot = guild.creation_slot.wrapping_add(bonus_value as u64);
                guild.total_exp = guild.total_exp.saturating_sub(10);
                msg!("Officer processing step {}", i);
            }
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitGuild<'info> {
    #[account(init, payer = master, space = 8 + 32 + 64 + 4 + 8 + 1 + 8)]
    pub guild: Account<'info, Guild>,
    #[account(mut)]
    pub master: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitMember<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 32 + 1 + 8 + 8 + 1 + 4)]
    pub member: Account<'info, Member>,
    #[account(mut)]
    pub guild: Account<'info, Guild>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// VULNERABLE: AccountInfoを制約なしで使用
#[derive(Accounts)]
pub struct UpdateGuildActivity<'info> {
    #[account(mut)]
    pub guild: Account<'info, Guild>,
    /// CHECK: 脆弱 - 型検証なし、同じアカウントを渡せる
    pub leader: AccountInfo<'info>,
    /// CHECK: 脆弱 - 型検証なし、同じアカウントを渡せる  
    pub officer: AccountInfo<'info>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Guild {
    pub master: Pubkey,
    pub name: String,
    pub member_count: u32,
    pub total_exp: u64,
    pub is_recruiting: bool,
    pub creation_slot: u64,
}

#[account]
pub struct Member {
    pub guild: Pubkey,
    pub owner: Pubkey,
    pub rank: GuildRank,
    pub contribution_points: u64,
    pub join_timestamp: i64,
    pub is_active: bool,
    pub member_id: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GuildRank {
    Recruit,
    Member,
    Veteran,
    Officer,
    Master,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Rank cosplay attempt blocked")]
    RankCosplayBlocked,
}