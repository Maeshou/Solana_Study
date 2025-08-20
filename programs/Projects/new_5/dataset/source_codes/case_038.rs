// 9. Reputation & Guild Reputation
declare_id!("K9L2M6N0P4Q8R1S5T9U3V7W1X5Y9Z3A7B1C5");

use anchor_lang::prelude::*;

#[program]
pub mod reputation_guild_insecure {
    use super::*;

    pub fn init_guild(ctx: Context<InitGuild>, guild_id: u32, name: String) -> Result<()> {
        let guild = &mut ctx.accounts.guild;
        guild.founder = ctx.accounts.founder.key();
        guild.guild_id = guild_id;
        guild.name = name;
        guild.reputation = 0;
        guild.member_count = 0;
        msg!("Guild '{}' initialized.", guild.name);
        Ok(())
    }

    pub fn init_member_profile(ctx: Context<InitMemberProfile>, profile_id: u64) -> Result<()> {
        let profile = &mut ctx.accounts.profile;
        let guild = &mut ctx.accounts.guild;
        
        profile.guild = guild.key();
        profile.profile_id = profile_id;
        profile.player = ctx.accounts.player.key();
        profile.personal_reputation = 0;
        profile.is_verified = false;
        
        guild.member_count = guild.member_count.saturating_add(1);
        msg!("Member profile {} created for guild {}.", profile.profile_id, guild.name);
        Ok(())
    }

    // Duplicate Mutable Account Vulnerability: profile_a と profile_b が同じアカウントであるかチェックしない
    pub fn update_reputation_profiles(ctx: Context<UpdateReputationProfiles>, score_diff: i64) -> Result<()> {
        let profile_a = &mut ctx.accounts.profile_a;
        let profile_b = &mut ctx.accounts.profile_b;

        let mut a_final_score: i64 = profile_a.personal_reputation as i64 + score_diff;
        let mut b_final_score: i64 = profile_b.personal_reputation as i64 - score_diff;

        let mut loop_count = 0;
        while loop_count < 2 {
            if a_final_score > b_final_score {
                a_final_score = a_final_score.saturating_add(100);
                b_final_score = b_final_score.saturating_sub(100);
                msg!("A has higher score, applying bonus to A and penalty to B.");
            } else {
                b_final_score = b_final_score.saturating_add(100);
                a_final_score = a_final_score.saturating_sub(100);
                msg!("B has higher or equal score, applying bonus to B and penalty to A.");
            }
            loop_count += 1;
        }

        if a_final_score > 0 {
            profile_a.personal_reputation = a_final_score as u64;
            profile_a.is_verified = true;
            ctx.accounts.guild.reputation = ctx.accounts.guild.reputation.saturating_add(profile_a.personal_reputation);
        } else {
            profile_a.personal_reputation = 0;
            profile_a.is_verified = false;
        }

        if b_final_score > 0 {
            profile_b.personal_reputation = b_final_score as u64;
            profile_b.is_verified = true;
            ctx.accounts.guild.reputation = ctx.accounts.guild.reputation.saturating_add(profile_b.personal_reputation);
        } else {
            profile_b.personal_reputation = 0;
            profile_b.is_verified = false;
        }
        
        msg!("Final reputation scores: A={}, B={}.", profile_a.personal_reputation, profile_b.personal_reputation);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitGuild<'info> {
    #[account(init, payer = founder, space = 8 + 32 + 4 + 32 + 8 + 4)]
    pub guild: Account<'info, Guild>,
    #[account(mut)]
    pub founder: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitMemberProfile<'info> {
    #[account(mut, has_one = guild)]
    pub guild: Account<'info, Guild>,
    #[account(init, payer = player, space = 8 + 32 + 8 + 32 + 8 + 1)]
    pub profile: Account<'info, MemberProfile>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateReputationProfiles<'info> {
    #[account(mut)]
    pub guild: Account<'info, Guild>,
    #[account(mut, has_one = guild)]
    pub profile_a: Account<'info, MemberProfile>,
    #[account(mut, has_one = guild)]
    pub profile_b: Account<'info, MemberProfile>,
}

#[account]
pub struct Guild {
    pub founder: Pubkey,
    pub guild_id: u32,
    pub name: String,
    pub reputation: u64,
    pub member_count: u32,
}

#[account]
pub struct MemberProfile {
    pub guild: Pubkey,
    pub profile_id: u64,
    pub player: Pubkey,
    pub personal_reputation: u64,
    pub is_verified: bool,
}

#[error_code]
pub enum ReputationError {
    #[msg("Guild is not active.")]
    GuildInactive,
}