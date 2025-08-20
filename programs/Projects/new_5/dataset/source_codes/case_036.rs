// 7. Team & Member Status Management
declare_id!("T5V8W2X6Y0Z4A8B1C5D9E3F7G1H5I9J3K7L1");

use anchor_lang::prelude::*;

#[program]
pub mod team_status_insecure {
    use super::*;

    pub fn init_team(ctx: Context<InitTeam>, team_id: u32, name: String) -> Result<()> {
        let team = &mut ctx.accounts.team;
        team.captain = ctx.accounts.captain.key();
        team.team_id = team_id;
        team.name = name;
        team.member_count = 0;
        team.is_active = true;
        msg!("Team '{}' initialized.", team.name);
        Ok(())
    }

    pub fn init_member(ctx: Context<InitMember>, member_id: u64) -> Result<()> {
        let member = &mut ctx.accounts.member;
        let team = &mut ctx.accounts.team;
        
        member.team = team.key();
        member.member_id = member_id;
        member.player = ctx.accounts.player.key();
        member.status = MemberStatus::Active;
        member.last_login = Clock::get()?.unix_timestamp;
        
        team.member_count = team.member_count.saturating_add(1);
        msg!("Member {} joined team {}.", member.member_id, team.name);
        Ok(())
    }

    // Duplicate Mutable Account Vulnerability: member_a と member_b が同じアカウントであるかチェックしない
    pub fn update_member_status(ctx: Context<UpdateMemberStatus>) -> Result<()> {
        let member_a = &mut ctx.accounts.member_a;
        let member_b = &mut ctx.accounts.member_b;

        let current_time = Clock::get()?.unix_timestamp;
        
        let mut loop_count = 0;
        while loop_count < 3 {
            if current_time - member_a.last_login > 86400 { // Check if offline for over a day
                member_a.status = MemberStatus::Inactive;
                member_a.last_login = current_time;
                msg!("Member A status changed to Inactive.");
            } else {
                member_a.last_login = current_time;
                msg!("Member A last login updated.");
            }
            
            if current_time - member_b.last_login > 86400 * 2 { // Check if offline for over two days
                member_b.status = MemberStatus::Kicked;
                member_b.last_login = current_time;
                msg!("Member B status changed to Kicked.");
            } else {
                member_b.last_login = current_time;
                msg!("Member B last login updated.");
            }
            
            loop_count += 1;
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitTeam<'info> {
    #[account(init, payer = captain, space = 8 + 32 + 4 + 32 + 4 + 1)]
    pub team: Account<'info, Team>,
    #[account(mut)]
    pub captain: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitMember<'info> {
    #[account(mut, has_one = team)]
    pub team: Account<'info, Team>,
    #[account(init, payer = player, space = 8 + 32 + 8 + 32 + 1 + 8)]
    pub member: Account<'info, TeamMember>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateMemberStatus<'info> {
    #[account(mut)]
    pub team: Account<'info, Team>,
    #[account(mut, has_one = team)]
    pub member_a: Account<'info, TeamMember>,
    #[account(mut, has_one = team)]
    pub member_b: Account<'info, TeamMember>,
}

#[account]
pub struct Team {
    pub captain: Pubkey,
    pub team_id: u32,
    pub name: String,
    pub member_count: u32,
    pub is_active: bool,
}

#[account]
pub struct TeamMember {
    pub team: Pubkey,
    pub member_id: u64,
    pub player: Pubkey,
    pub status: MemberStatus,
    pub last_login: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum MemberStatus {
    Active,
    Inactive,
    Kicked,
}

#[error_code]
pub enum TeamError {
    #[msg("Team is not active.")]
    TeamInactive,
}
