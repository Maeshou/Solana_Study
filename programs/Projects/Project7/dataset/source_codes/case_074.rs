use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, MintTo, Token, TokenAccount, Mint};

declare_id!("Build8ProjA7kXq2Wm4Qy6Vt8Rb0Lc3Za5Hd7Q308");

#[program]
pub mod building_project_progress_v1 {
    use super::*;

    pub fn init_project(ctx: Context<InitProject>, milestone_step_input: u64) -> Result<()> {
        let project = &mut ctx.accounts.project;
        project.owner = ctx.accounts.owner.key();
        project.milestone_step = milestone_step_input;
        if project.milestone_step < 5 { project.milestone_step = 5; }
        project.material_burned = 1;
        project.progress_points = 0;
        project.milestones_completed = 0;
        Ok(())
    }

    pub fn act_build(ctx: Context<ActBuild>, material_units: u64) -> Result<()> {
        let project = &mut ctx.accounts.project;

        let mut burn_units: u64 = material_units;
        if burn_units < 1 { burn_units = 1; }
        token::burn(ctx.accounts.burn_ctx(), burn_units)?;

        // 進捗ポイント加算：消費量に応じて段階的加点
        let mut added_points: u64 = burn_units / 2 + 1;
        let mut step_counter: u8 = 0;
        while step_counter < 3 {
            added_points = added_points + step_counter as u64;
            step_counter = step_counter + 1;
        }
        project.progress_points = project.progress_points + added_points;
        project.material_burned = project.material_burned + burn_units;

        // マイルストーン達成時にボーナス発行
        let mut bonus_units: u64 = 0;
        if project.progress_points >= project.milestone_step {
            bonus_units = project.milestone_step / 3 + 1;
            token::mint_to(ctx.accounts.mint_ctx(), bonus_units)?;
            project.progress_points = project.progress_points - project.milestone_step;
            project.milestones_completed = project.milestones_completed + 1;
        }
        if project.milestones_completed % 5 == 0 && project.milestones_completed > 0 {
            let additional = bonus_units + 2;
            token::mint_to(ctx.accounts.mint_ctx(), additional)?;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitProject<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8 + 8)]
    pub project: Account<'info, ProjectState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActBuild<'info> {
    #[account(mut, has_one = owner)]
    pub project: Account<'info, ProjectState>,
    pub owner: Signer<'info>,

    pub material_mint: Account<'info, Mint>,
    #[account(mut)]
    pub material_vault: Account<'info, TokenAccount>,

    pub bonus_mint: Account<'info, Mint>,
    #[account(mut)]
    pub bonus_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActBuild<'info> {
    pub fn burn_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let call = Burn { mint: self.material_mint.to_account_info(), from: self.material_vault.to_account_info(), authority: self.owner.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), call)
    }
    pub fn mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let call = MintTo { mint: self.bonus_mint.to_account_info(), to: self.bonus_vault.to_account_info(), authority: self.owner.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), call)
    }
}
#[account]
pub struct ProjectState {
    pub owner: Pubkey,
    pub milestone_step: u64,
    pub material_burned: u64,
    pub progress_points: u64,
    pub milestones_completed: u64,
}
