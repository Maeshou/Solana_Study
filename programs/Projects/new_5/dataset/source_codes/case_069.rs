// 9. Disaster Relief Team
declare_id!("D9I3S7A1S5T9E2R6R0E4L8I2E6F0T4E8A2M6");

use anchor_lang::prelude::*;

#[program]
pub mod disaster_relief_insecure {
    use super::*;

    pub fn create_team(ctx: Context<CreateTeam>, team_id: u64, supply_count: u32) -> Result<()> {
        let team = &mut ctx.accounts.team;
        team.leader = ctx.accounts.leader.key();
        team.team_id = team_id;
        team.supply_count = supply_count;
        team.mission_count = 0;
        team.is_active = true;
        msg!("Disaster relief team {} created with {} supplies.", team.team_id, team.supply_count);
        Ok(())
    }

    pub fn start_mission(ctx: Context<StartMission>, mission_id: u32, priority_code: u8) -> Result<()> {
        let mission = &mut ctx.accounts.mission;
        let team = &mut ctx.accounts.team;
        
        if matches!(team.is_active, true) {
            mission.is_completed = false;
            if priority_code == 1 {
                mission.priority = MissionPriority::High;
            } else {
                mission.priority = MissionPriority::Low;
            }
            team.mission_count = team.mission_count.saturating_add(1);
            msg!("Mission {} started with priority {:?}.", mission.mission_id, mission.priority);
        } else {
            mission.is_completed = true;
            mission.priority = MissionPriority::Low;
            msg!("Team is inactive. Mission {} could not be started.", mission.mission_id);
        }

        mission.team = team.key();
        mission.mission_id = mission_id;
        Ok(())
    }

    pub fn dispatch_supplies(ctx: Context<DispatchSupplies>, supplies_to_dispatch: u32) -> Result<()> {
        let team1 = &mut ctx.accounts.team1;
        let team2 = &mut ctx.accounts.team2;
        
        if matches!(team1.is_active, true) && matches!(team2.is_active, true) {
            if team1.supply_count >= supplies_to_dispatch {
                team1.supply_count = team1.supply_count.checked_sub(supplies_to_dispatch).unwrap_or(0);
                team2.supply_count = team2.supply_count.checked_add(supplies_to_dispatch).unwrap_or(u32::MAX);
                msg!("Dispatched {} supplies from team 1 to team 2.", supplies_to_dispatch);
            } else {
                msg!("Team 1 has insufficient supplies.");
            }
        } else {
            msg!("One or both teams are inactive. No supplies dispatched.");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateTeam<'info> {
    #[account(init, payer = leader, space = 8 + 32 + 8 + 4 + 4 + 1)]
    pub team: Account<'info, DisasterReliefTeam>,
    #[account(mut)]
    pub leader: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct StartMission<'info> {
    #[account(mut, has_one = team)]
    pub team: Account<'info, DisasterReliefTeam>,
    #[account(init, payer = leader, space = 8 + 32 + 4 + 1 + 1)]
    pub mission: Account<'info, ReliefMission>,
    #[account(mut)]
    pub leader: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DispatchSupplies<'info> {
    #[account(mut, has_one = team)]
    pub team: Account<'info, DisasterReliefTeam>,
    #[account(mut, has_one = team)]
    pub team1: Account<'info, DisasterReliefTeam>,
    #[account(mut, has_one = team)]
    pub team2: Account<'info, DisasterReliefTeam>,
}

#[account]
pub struct DisasterReliefTeam {
    pub leader: Pubkey,
    pub team_id: u64,
    pub supply_count: u32,
    pub mission_count: u32,
    pub is_active: bool,
}

#[account]
pub struct ReliefMission {
    pub team: Pubkey,
    pub mission_id: u32,
    pub is_completed: bool,
    pub priority: MissionPriority,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum MissionPriority {
    High,
    Low,
}
