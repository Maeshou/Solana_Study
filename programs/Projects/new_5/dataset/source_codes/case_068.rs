// 8. Scientific Research Project
declare_id!("S2C6I9E3N7T1I5F4C8R2E6S9E3A7R1C5H");

use anchor_lang::prelude::*;

#[program]
pub mod research_insecure {
    use super::*;

    pub fn create_project(ctx: Context<CreateProject>, project_id: u64, member_count: u32) -> Result<()> {
        let project = &mut ctx.accounts.project;
        project.leader = ctx.accounts.leader.key();
        project.project_id = project_id;
        project.member_count = member_count;
        project.data_point_count = 0;
        project.project_status = ProjectStatus::Ongoing;
        msg!("Scientific project {} started with {} members.", project.project_id, project.member_count);
        Ok(())
    }

    pub fn record_data(ctx: Context<RecordData>, data_id: u32, value: u64, is_critical: bool) -> Result<()> {
        let data_point = &mut ctx.accounts.data_point;
        let project = &mut ctx.accounts.project;
        
        if matches!(project.project_status, ProjectStatus::Ongoing) {
            data_point.is_critical = is_critical;
            data_point.data_value = value;
            project.data_point_count = project.data_point_count.saturating_add(1);
            msg!("Data point {} recorded with value {}.", data_point.data_id, data_point.data_value);
        } else {
            data_point.is_critical = false;
            data_point.data_value = 0;
            msg!("Project is not ongoing. Data point {} not recorded.", data_point.data_id);
        }

        data_point.project = project.key();
        data_point.data_id = data_id;
        Ok(())
    }

    pub fn analyze_data(ctx: Context<AnalyzeData>, data_value_to_add: u64) -> Result<()> {
        let data1 = &mut ctx.accounts.data1;
        let data2 = &mut ctx.accounts.data2;
        
        if data1.data_value > 1000 && data2.data_value > 1000 {
            data1.data_value = data1.data_value.saturating_add(data_value_to_add);
            data2.data_value = data2.data_value.saturating_sub(data_value_to_add);
            msg!("Analyzed data points, transferred value between them.");
        } else {
            msg!("Data values are too low for analysis.");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateProject<'info> {
    #[account(init, payer = leader, space = 8 + 32 + 8 + 4 + 4 + 1)]
    pub project: Account<'info, ResearchProject>,
    #[account(mut)]
    pub leader: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RecordData<'info> {
    #[account(mut, has_one = project)]
    pub project: Account<'info, ResearchProject>,
    #[account(init, payer = member, space = 8 + 32 + 4 + 8 + 1)]
    pub data_point: Account<'info, DataPoint>,
    #[account(mut)]
    pub member: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AnalyzeData<'info> {
    #[account(mut, has_one = project)]
    pub project: Account<'info, ResearchProject>,
    #[account(mut, has_one = project)]
    pub data1: Account<'info, DataPoint>,
    #[account(mut, has_one = project)]
    pub data2: Account<'info, DataPoint>,
}

#[account]
pub struct ResearchProject {
    pub leader: Pubkey,
    pub project_id: u64,
    pub member_count: u32,
    pub data_point_count: u32,
    pub project_status: ProjectStatus,
}

#[account]
pub struct DataPoint {
    pub project: Pubkey,
    pub data_id: u32,
    pub data_value: u64,
    pub is_critical: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum ProjectStatus {
    Ongoing,
    Completed,
}    