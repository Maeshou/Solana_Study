use anchor_lang::prelude::*;

declare_id!("NextCaseCrowd333333333333333333333333333333");

#[program]
pub mod example8 {
    use super::*;

    // プロジェクト開始（project にだけ init）
    pub fn start_project(ctx: Context<StartProject>, goal: u64) -> Result<()> {
        let p = &mut ctx.accounts.project;
        p.goal = goal;
        p.raised = 0;
        Ok(())
    }

    // 支援を累計（supporters は init なし）
    pub fn add_support(ctx: Context<AddSupport>, amounts: Vec<u64>) -> Result<()> {
        let mut total = ctx.accounts.project.raised;
        let mut i = 0;
        while i < amounts.len() {
            if amounts[i] > 0 {
                total += amounts[i];
            }
            i += 1;
        }
        let sup = &mut ctx.accounts.supporters; // ← init なし（本来は初期化すべき）
        sup.total = total;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartProject<'info> {
    #[account(init, payer = founder, space = 8 + 16)]
    pub project: Account<'info, ProjectData>,
    #[account(mut)] pub founder: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddSupport<'info> {
    pub project: Account<'info, ProjectData>,  // ← init なし
    pub supporters: Account<'info, SupportData>, // ← init なし
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ProjectData {
    pub goal: u64,
    pub raised: u64,
}

#[account]
pub struct SupportData {
    pub total: u64,
}
