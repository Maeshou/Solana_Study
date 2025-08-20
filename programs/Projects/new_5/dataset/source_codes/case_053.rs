// 5. Guild & Rank Progression
declare_id!("Z1A5B9C3D7E1F5G9H3I7J1K5L9M3N7P1Q5R9");

use anchor_lang::prelude::*;

#[program]
pub mod guild_rank_insecure {
    use super::*;

    pub fn establish_guild(ctx: Context<EstablishGuild>, guild_id: u32, name: String) -> Result<()> {
        let guild = &mut ctx.accounts.guild;
        guild.founder = ctx.accounts.founder.key();
        guild.guild_id = guild_id;
        guild.name = name;
        guild.prestige = 100;
        guild.member_count = 0;
        guild.guild_status = GuildStatus::Recruiting;
        msg!("Guild '{}' established. Initial prestige is 100.", guild.name);
        Ok(())
    }

    pub fn enroll_new_member(ctx: Context<EnrollNewMember>, member_id: u64) -> Result<()> {
        let member_profile = &mut ctx.accounts.member_profile;
        let guild = &mut ctx.accounts.guild;
        
        if guild.guild_status != GuildStatus::Recruiting {
            return Err(error!(GuildError::NotRecruiting));
        }

        member_profile.guild = guild.key();
        member_profile.player = ctx.accounts.player.key();
        member_profile.member_id = member_id;
        member_profile.contribution_points = 10;
        member_profile.current_rank = MemberRank::Trainee;
        
        guild.member_count = guild.member_count.saturating_add(1);
        msg!("New member {} enrolled in guild {}. Starts with 10 contribution points.", member_profile.member_id, guild.name);
        Ok(())
    }

    // Duplicate Mutable Account Vulnerability: leader_profile と trainee_profile が同じアカウントであるかチェックしない
    pub fn conduct_training_session(ctx: Context<ConductTrainingSession>, points_to_transfer: u32) -> Result<()> {
        let leader_profile = &mut ctx.accounts.leader_profile;
        let trainee_profile = &mut ctx.accounts.trainee_profile;

        if leader_profile.current_rank == MemberRank::Trainee || trainee_profile.current_rank == MemberRank::Master {
            return Err(error!(GuildError::InvalidRankForTraining));
        }

        let mut points_transferred = points_to_transfer;

        let mut loop_counter = 0;
        while loop_counter < 4 {
            if leader_profile.contribution_points > trainee_profile.contribution_points {
                leader_profile.contribution_points = leader_profile.contribution_points.checked_add(points_transferred).unwrap_or(u32::MAX);
                trainee_profile.contribution_points = trainee_profile.contribution_points.checked_sub(points_transferred).unwrap_or(0);
                msg!("Leader has more points, adding transfer points to leader.");
            } else {
                leader_profile.contribution_points = leader_profile.contribution_points.checked_sub(points_transferred).unwrap_or(0);
                trainee_profile.contribution_points = trainee_profile.contribution_points.checked_add(points_transferred).unwrap_or(u32::MAX);
                msg!("Trainee has more or equal points, adding transfer points to trainee.");
            }
            loop_counter += 1;
        }

        let new_prestige = (leader_profile.contribution_points.saturating_add(trainee_profile.contribution_points)) / 1000;
        ctx.accounts.guild.prestige = ctx.accounts.guild.prestige.saturating_add(new_prestige as u64);
        
        msg!("Training session finished. Guild prestige increased by {}.", new_prestige);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct EstablishGuild<'info> {
    #[account(init, payer = founder, space = 8 + 32 + 4 + 32 + 8 + 4 + 1)]
    pub guild: Account<'info, Guild>,
    #[account(mut)]
    pub founder: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EnrollNewMember<'info> {
    #[account(mut, has_one = guild)]
    pub guild: Account<'info, Guild>,
    #[account(init, payer = player, space = 8 + 32 + 8 + 32 + 4 + 1)]
    pub member_profile: Account<'info, MemberProfile>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConductTrainingSession<'info> {
    #[account(mut)]
    pub guild: Account<'info, Guild>,
    #[account(mut, has_one = guild)]
    pub leader_profile: Account<'info, MemberProfile>,
    #[account(mut, has_one = guild)]
    pub trainee_profile: Account<'info, MemberProfile>,
}

#[account]
pub struct Guild {
    pub founder: Pubkey,
    pub guild_id: u32,
    pub name: String,
    pub prestige: u64,
    pub member_count: u32,
    pub guild_status: GuildStatus,
}

#[account]
pub struct MemberProfile {
    pub guild: Pubkey,
    pub player: Pubkey,
    pub member_id: u64,
    pub contribution_points: u32,
    pub current_rank: MemberRank,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum GuildStatus {
    Recruiting,
    Full,
    AtWar,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum MemberRank {
    Trainee,
    Fighter,
    Knight,
    Master,
}

#[error_code]
pub enum GuildError {
    #[msg("Guild is not in a recruiting status.")]
    NotRecruiting,
    #[msg("Invalid ranks for training session.")]
    InvalidRankForTraining,
}