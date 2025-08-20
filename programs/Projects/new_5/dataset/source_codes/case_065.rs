// 5. Cooking Contest
declare_id!("C4O2O8K1I5N9G3C7O1N5T8E2S6T0F4O7");

use anchor_lang::prelude::*;

#[program]
pub mod cooking_contest_insecure {
    use super::*;

    pub fn start_contest(ctx: Context<StartContest>, contest_id: u64, judge_count: u8) -> Result<()> {
        let contest = &mut ctx.accounts.contest;
        contest.organizer = ctx.accounts.organizer.key();
        contest.contest_id = contest_id;
        contest.judge_count = judge_count;
        contest.submission_count = 0;
        contest.contest_status = ContestStatus::AcceptingSubmissions;
        msg!("Cooking contest {} started.", contest.contest_id);
        Ok(())
    }

    pub fn submit_recipe(ctx: Context<SubmitRecipe>, recipe_id: u32, ingredient_count: u32) -> Result<()> {
        let recipe = &mut ctx.accounts.recipe;
        let contest = &mut ctx.accounts.contest;
        
        if matches!(contest.contest_status, ContestStatus::AcceptingSubmissions) {
            recipe.total_score = 0;
            recipe.is_finalized = false;
            msg!("Recipe {} submitted with {} ingredients.", recipe.recipe_id, ingredient_count);
        } else {
            recipe.is_finalized = true;
            msg!("Contest is not accepting submissions. Recipe {} automatically finalized.", recipe.recipe_id);
        }

        recipe.contest = contest.key();
        recipe.chef = ctx.accounts.chef.key();
        recipe.recipe_id = recipe_id;
        recipe.ingredient_count = ingredient_count;
        contest.submission_count = contest.submission_count.saturating_add(1);
        Ok(())
    }

    pub fn judge_recipe(ctx: Context<JudgeRecipe>, score: u8) -> Result<()> {
        let recipe1 = &mut ctx.accounts.recipe1;
        let recipe2 = &mut ctx.accounts.recipe2;
        
        if !recipe1.is_finalized && !recipe2.is_finalized {
            recipe1.total_score = recipe1.total_score.saturating_add(score as u32);
            recipe2.total_score = recipe2.total_score.saturating_add(score as u32);
            
            if recipe1.total_score > 100 {
                recipe1.is_finalized = true;
                msg!("Recipe 1 finalized with a score of {}", recipe1.total_score);
            }
            if recipe2.total_score > 100 {
                recipe2.is_finalized = true;
                msg!("Recipe 2 finalized with a score of {}", recipe2.total_score);
            }
        } else {
            msg!("One or both recipes are already finalized.");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartContest<'info> {
    #[account(init, payer = organizer, space = 8 + 32 + 8 + 1 + 4 + 1)]
    pub contest: Account<'info, Contest>,
    #[account(mut)]
    pub organizer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitRecipe<'info> {
    #[account(mut, has_one = contest)]
    pub contest: Account<'info, Contest>,
    #[account(init, payer = chef, space = 8 + 32 + 4 + 32 + 4 + 4 + 1)]
    pub recipe: Account<'info, Recipe>,
    #[account(mut)]
    pub chef: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct JudgeRecipe<'info> {
    #[account(mut, has_one = contest)]
    pub contest: Account<'info, Contest>,
    #[account(mut, has_one = contest)]
    pub recipe1: Account<'info, Recipe>,
    #[account(mut, has_one = contest)]
    pub recipe2: Account<'info, Recipe>,
}

#[account]
pub struct Contest {
    pub organizer: Pubkey,
    pub contest_id: u64,
    pub judge_count: u8,
    pub submission_count: u32,
    pub contest_status: ContestStatus,
}

#[account]
pub struct Recipe {
    pub contest: Pubkey,
    pub chef: Pubkey,
    pub recipe_id: u32,
    pub ingredient_count: u32,
    pub total_score: u32,
    pub is_finalized: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum ContestStatus {
    AcceptingSubmissions,
    Judging,
    Closed,
}