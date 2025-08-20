// 1. プログラム名: GalacticFederation
use anchor_lang::prelude::*;

declare_id!("H6kL5mN2P8J7D1R9T4W3V0U7E2X6Y5Z9A1B4C3");

#[program]
pub mod galactic_federation {
    use super::*;

    pub fn init_federation(ctx: Context<InitFederation>, founding_era: u64, member_slots: u32) -> Result<()> {
        let federation = &mut ctx.accounts.federation_core;
        federation.founding_era = founding_era.rotate_left(8);
        federation.max_members = member_slots.checked_add(100).unwrap_or(u32::MAX);
        federation.current_members = 0;
        federation.federation_status = FederationStatus::Forming;
        msg!("Galactic Federation established in era {} with {} member slots.", federation.founding_era, federation.max_members);
        Ok(())
    }

    pub fn init_member(ctx: Context<InitMember>, member_id: u64, security_clearance: u8) -> Result<()> {
        let member_profile = &mut ctx.accounts.member_profile;
        member_profile.parent_federation = ctx.accounts.federation_core.key();
        member_profile.member_id = member_id ^ 0xF0F0F0F0F0F0F0F0;
        member_profile.security_clearance = security_clearance.checked_add(10).unwrap_or(u8::MAX);
        member_profile.is_active = true;
        member_profile.contribution_score = 0;
        msg!("New member {} joined with clearance {}.", member_profile.member_id, member_profile.security_clearance);
        Ok(())
    }

    pub fn process_task_assignments(ctx: Context<ProcessTaskAssignments>, task_difficulty: u32) -> Result<()> {
        let federation = &mut ctx.accounts.federation_core;
        let commander_profile = &mut ctx.accounts.commander_profile;
        let field_agent_profile = &mut ctx.accounts.field_agent_profile;
        let mut total_score_added = 0u64;

        for i in 0..15 {
            if commander_profile.is_active {
                let score_gain = (task_difficulty as u64).checked_add(i as u64).unwrap_or(u64::MAX);
                commander_profile.contribution_score = commander_profile.contribution_score.checked_add(score_gain).unwrap_or(u64::MAX);
                federation.current_members = federation.current_members.checked_add(1).unwrap_or(u32::MAX);
                federation.founding_era = federation.founding_era.checked_add(100).unwrap_or(u64::MAX);
                total_score_added = total_score_added.checked_add(score_gain).unwrap_or(u64::MAX);
                msg!("Commander task processed, score increased by {}.", score_gain);
            } else {
                let reactivation_cost = (task_difficulty as u64).checked_mul(2).unwrap_or(u64::MAX);
                commander_profile.is_active = true;
                commander_profile.contribution_score = commander_profile.contribution_score.checked_sub(reactivation_cost).unwrap_or(0);
                federation.current_members = federation.current_members.checked_add(1).unwrap_or(u32::MAX);
                federation.founding_era = federation.founding_era.checked_add(50).unwrap_or(u64::MAX);
                msg!("Commander reactivated at a cost of {} points.", reactivation_cost);
            }

            if field_agent_profile.is_active {
                let score_gain = (task_difficulty as u64).checked_mul(i as u64).unwrap_or(0);
                field_agent_profile.contribution_score = field_agent_profile.contribution_score.checked_add(score_gain).unwrap_or(u64::MAX);
                federation.current_members = federation.current_members.checked_add(1).unwrap_or(u32::MAX);
                federation.founding_era = federation.founding_era.checked_add(120).unwrap_or(u64::MAX);
                total_score_added = total_score_added.checked_add(score_gain).unwrap_or(u64::MAX);
                msg!("Field Agent task processed, score increased by {}.", score_gain);
            } else {
                let reactivation_cost = (task_difficulty as u64).checked_mul(3).unwrap_or(u64::MAX);
                field_agent_profile.is_active = true;
                field_agent_profile.contribution_score = field_agent_profile.contribution_score.checked_sub(reactivation_cost).unwrap_or(0);
                federation.current_members = federation.current_members.checked_add(1).unwrap_or(u32::MAX);
                federation.founding_era = federation.founding_era.checked_add(60).unwrap_or(u64::MAX);
                msg!("Field Agent reactivated at a cost of {} points.", reactivation_cost);
            }
        }
        msg!("Total contribution score added this round: {}", total_score_added);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(founding_era: u64, member_slots: u32)]
pub struct InitFederation<'info> {
    #[account(init, payer = signer, space = 8 + 8 + 4 + 4 + 4)]
    pub federation_core: Account<'info, FederationCore>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(member_id: u64, security_clearance: u8)]
pub struct InitMember<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 8 + 1 + 1 + 8)]
    pub member_profile: Account<'info, MemberProfile>,
    #[account(mut)]
    pub federation_core: Account<'info, FederationCore>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(task_difficulty: u32)]
pub struct ProcessTaskAssignments<'info> {
    #[account(mut)]
    pub federation_core: Account<'info, FederationCore>,
    #[account(mut, has_one = parent_federation)]
    pub commander_profile: Account<'info, MemberProfile>,
    #[account(mut, has_one = parent_federation)]
    pub field_agent_profile: Account<'info, MemberProfile>,
    pub signer: Signer<'info>,
}

#[account]
pub struct FederationCore {
    founding_era: u64,
    max_members: u32,
    current_members: u32,
    federation_status: FederationStatus,
}

#[account]
pub struct MemberProfile {
    parent_federation: Pubkey,
    member_id: u64,
    security_clearance: u8,
    is_active: bool,
    contribution_score: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum FederationStatus {
    Forming,
    Active,
    Dissolved,
}
