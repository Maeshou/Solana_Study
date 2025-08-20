// 5. ワークフロー＋進捗管理
use anchor_lang::prelude::*;
declare_id!("WFLW111122223333444455556666777788");

#[program]
pub mod misinit_workflow_v4 {
    use super::*;

    pub fn start_workflow(ctx: Context<StartWorkflow>, name: String) -> Result<()> {
        let wf = &mut ctx.accounts.workflow;
        wf.name = name;
        wf.stage = 0;
        wf.created_at = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn advance_stage(ctx: Context<StartWorkflow>) -> Result<()> {
        let wf = &mut ctx.accounts.workflow;
        require!(wf.stage < wf.max_stage, ErrorCode4::MaxStageReached);
        wf.stage += 1;
        let log = &mut ctx.accounts.workflow_log;
        log.entries.push((wf.stage, Clock::get()?.unix_timestamp));
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartWorkflow<'info> {
    #[account(init, payer = signer, space = 8 + (4+32) + 1 + 1 + 8)] pub workflow: Account<'info, WorkflowData>,
    #[account(mut)] pub workflow_log: Account<'info, WorkflowLog>,
    #[account(mut)] pub signer: Signer<'info>, pub system_program: Program<'info, System>,
}

#[account]
pub struct WorkflowData { pub name:String, pub stage:u8, pub max_stage:u8, pub created_at:i64 }
#[account]
pub struct WorkflowLog { pub entries: Vec<(u8,i64)> }
#[error_code] pub enum ErrorCode4 { #[msg("最大ステージに到達しています。")] MaxStageReached }
