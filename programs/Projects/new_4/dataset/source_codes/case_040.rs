// 5. ワークフロー＋進捗管理（Clockなし）
use anchor_lang::prelude::*;
declare_id!("WFLW888899990000AAAA1111BBBB2222");

#[program]
pub mod misinit_workflow_no_clock {
    use super::*;

    pub fn start_workflow(
        ctx: Context<StartWorkflow>,
        name: String,
        max_stage: u8,
    ) -> Result<()> {
        let wf = &mut ctx.accounts.workflow;
        wf.name = name;
        wf.stage = 0;
        wf.max = max_stage;
        Ok(())
    }

    pub fn advance_stage(ctx: Context<StartWorkflow>) -> Result<()> {
        let wf = &mut ctx.accounts.workflow;
        require!(wf.stage < wf.max, ErrorCode5::MaxReached);
        wf.stage = wf.stage.checked_add(1).unwrap();
        let log = &mut ctx.accounts.workflow_log;
        log.stages.push(wf.stage);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartWorkflow<'info> {
    #[account(init, payer = signer, space = 8+(4+32)+1+1)] pub workflow: Account<'info, WorkflowData>,
    #[account(mut)] pub workflow_log: Account<'info, WorkflowLog>,
    #[account(mut)] pub signer: Signer<'info>, pub system_program: Program<'info, System>,
}

#[account]
pub struct WorkflowData { pub name:String, pub stage:u8, pub max:u8 }
#[account]
pub struct WorkflowLog { pub stages: Vec<u8> }

#[error_code]
pub enum ErrorCode5 { #[msg("これ以上ステージを進められません。")] MaxReached }
