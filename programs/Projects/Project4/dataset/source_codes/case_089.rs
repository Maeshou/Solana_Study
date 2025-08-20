use anchor_lang::prelude::*;

declare_id!("SafeEx35SessionPipe111111111111111111111111");

#[program]
pub mod example35 {
    use super::*;

    pub fn init_session_pipe(
        ctx: Context<InitSessionPipe>,
        steps: u8,
    ) -> Result<()> {
        let p = &mut ctx.accounts.pipe;
        p.steps         = steps;
        p.completed     = 0;
        p.error_count   = 0;

        // 初期ステップを段階的に完了
        let mut i = 0u8;
        while i < steps / 2 {
            p.completed = p.completed.saturating_add(1);
            i += 1;
        }
        Ok(())
    }

    pub fn advance_step(
        ctx: Context<AdvanceStep>,
        success: bool,
    ) -> Result<()> {
        let p = &mut ctx.accounts.pipe;
        if success {
            if p.completed < p.steps {
                p.completed = p.completed.saturating_add(1);
            }
        } else {
            p.error_count = p.error_count.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitSessionPipe<'info> {
    #[account(init, payer = user, space = 8 + 1 + 1 + 1)]
    pub pipe: Account<'info, SessionPipelineData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AdvanceStep<'info> {
    #[account(mut)] pub pipe: Account<'info, SessionPipelineData>,
}

#[account]
pub struct SessionPipelineData {
    pub steps:       u8,
    pub completed:   u8,
    pub error_count: u8,
}
