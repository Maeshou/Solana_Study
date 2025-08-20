use anchor_lang::prelude::*;

declare_id!("SafeEx13Pipeline111111111111111111111111111");

#[program]
pub mod example13 {
    use super::*;

    /// パイプラインを初期化
    pub fn init_pipeline(
        ctx: Context<InitPipeline>,
        stages: u8,
    ) -> Result<()> {
        let p = &mut ctx.accounts.pipeline;
        p.stages    = stages;
        p.completed = 0;
        p.errors    = 0;
        Ok(())
    }

    /// ステージを実行しエラーをカウント
    pub fn process_stage(
        ctx: Context<ProcessStage>,
        success: bool,
    ) -> Result<()> {
        let p = &mut ctx.accounts.pipeline;
        // 成功時は completed +1、失敗時は errors +1
        if success {
            if p.completed < p.stages {
                p.completed += 1;
            }
        } else {
            p.errors += 1;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPipeline<'info> {
    #[account(init, payer = user, space = 8 + 1*3)]
    pub pipeline: Account<'info, PipelineData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessStage<'info> {
    #[account(mut)] pub pipeline: Account<'info, PipelineData>,
}

#[account]
pub struct PipelineData {
    pub stages:    u8,
    pub completed: u8,
    pub errors:    u8,
}
