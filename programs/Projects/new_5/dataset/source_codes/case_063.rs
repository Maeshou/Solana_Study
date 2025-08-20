// 3. Space Exploration Guild
declare_id!("S7P2A5C9E3X7P1L4O8R2A6T0I9O3N7G1");

use anchor_lang::prelude::*;

#[program]
pub mod space_guild_insecure {
    use super::*;

    pub fn launch_expedition(ctx: Context<LaunchExpedition>, expedition_id: u64, vessel_name: String) -> Result<()> {
        let expedition = &mut ctx.accounts.expedition;
        expedition.leader = ctx.accounts.leader.key();
        expedition.expedition_id = expedition_id;
        expedition.vessel_name = vessel_name;
        expedition.crew_count = 1;
        expedition.expedition_status = ExpeditionStatus::OnMission;
        msg!("Expedition {} launched with vessel {}.", expedition.expedition_id, expedition.vessel_name);
        Ok(())
    }

    pub fn join_crew(ctx: Context<JoinCrew>, crew_member_id: u32, skill_level: u8, role_code: u8) -> Result<()> {
        let crew_member = &mut ctx.accounts.crew_member;
        let expedition = &mut ctx.accounts.expedition;
        
        if matches!(expedition.expedition_status, ExpeditionStatus::OnMission) {
            crew_member.expedition_id = expedition.expedition_id;
            crew_member.skill_level = skill_level;
            if role_code == 1 {
                crew_member.role = CrewRole::Pilot;
            } else {
                crew_member.role = CrewRole::Engineer;
            }
            expedition.crew_count = expedition.crew_count.saturating_add(1);
            msg!("Crew member {} joined the expedition.", crew_member.member_id);
        } else {
            msg!("Expedition is not on a mission. Cannot join crew.");
        }
        
        crew_member.member_id = crew_member_id;
        crew_member.wallet_address = ctx.accounts.new_member.key();
        Ok(())
    }

    pub fn transfer_resources(ctx: Context<TransferResources>, amount: u32) -> Result<()> {
        let member1 = &mut ctx.accounts.member1;
        let member2 = &mut ctx.accounts.member2;
        
        if member1.expedition_id == member2.expedition_id {
            member1.skill_level = member1.skill_level.saturating_sub(amount as u8);
            member2.skill_level = member2.skill_level.saturating_add(amount as u8);
            msg!("Transferred {} skill level from member1 to member2.", amount);
        } else {
            msg!("Crew members are not on the same expedition.");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LaunchExpedition<'info> {
    #[account(init, payer = leader, space = 8 + 32 + 8 + 32 + 4 + 1)]
    pub expedition: Account<'info, Expedition>,
    #[account(mut)]
    pub leader: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct JoinCrew<'info> {
    #[account(mut, has_one = expedition_id)]
    pub expedition: Account<'info, Expedition>,
    #[account(init, payer = new_member, space = 8 + 4 + 32 + 1 + 1 + 8)]
    pub crew_member: Account<'info, CrewMember>,
    #[account(mut)]
    pub new_member: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferResources<'info> {
    #[account(mut)]
    pub expedition: Account<'info, Expedition>,
    #[account(mut, has_one = expedition_id)]
    pub member1: Account<'info, CrewMember>,
    #[account(mut, has_one = expedition_id)]
    pub member2: Account<'info, CrewMember>,
}

#[account]
pub struct Expedition {
    pub leader: Pubkey,
    pub expedition_id: u64,
    pub vessel_name: String,
    pub crew_count: u32,
    pub expedition_status: ExpeditionStatus,
}

#[account]
pub struct CrewMember {
    pub expedition_id: u64,
    pub member_id: u32,
    pub wallet_address: Pubkey,
    pub skill_level: u8,
    pub role: CrewRole,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum ExpeditionStatus {
    OnMission,
    Returned,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum CrewRole {
    Pilot,
    Engineer,
}
