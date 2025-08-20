// 3. Quest & Reward System
declare_id!("E4G8J1K5P9L3M7N2Q6R0T4U8V2W6X0Y4Z8A2B6C0");

use anchor_lang::prelude::*;

#[program]
pub mod quest_reward_insecure {
    use super::*;

    pub fn init_quest_board(ctx: Context<InitQuestBoard>, name: String) -> Result<()> {
        let board = &mut ctx.accounts.board;
        board.admin = ctx.accounts.admin.key();
        board.name = name;
        board.total_quests = 0;
        board.is_open = true;
        msg!("Quest board '{}' initialized.", board.name);
        Ok(())
    }

    pub fn init_quest(ctx: Context<InitQuest>, quest_id: u32, difficulty: u8, reward_amount: u64) -> Result<()> {
        let quest = &mut ctx.accounts.quest;
        let board = &mut ctx.accounts.board;
        
        quest.quest_board = board.key();
        quest.quest_id = quest_id;
        quest.difficulty = difficulty;
        quest.reward_amount = reward_amount;
        quest.is_completed = false;
        
        board.total_quests = board.total_quests.saturating_add(1);
        msg!("Quest {} added to board {}.", quest.quest_id, board.name);
        Ok(())
    }

    // Duplicate Mutable Account Vulnerability: quest_a と quest_b が同じアカウントであるかチェックしない
    pub fn complete_quests(ctx: Context<CompleteQuests>, scores: Vec<u8>) -> Result<()> {
        let quest_a = &mut ctx.accounts.quest_a;
        let quest_b = &mut ctx.accounts.quest_b;
        
        if !ctx.accounts.board.is_open {
            return Err(ErrorCode::BoardClosed.into());
        }

        let mut points_a: u64 = 0;
        let mut points_b: u64 = 0;
        let mut score_sum = 0;

        for score in scores.iter() {
            score_sum = score_sum.saturating_add(*score);
            let success_factor = if *score > 80 { 1.5 } else { 1.0 };
            
            if quest_a.difficulty > quest_b.difficulty {
                points_a = points_a.saturating_add((quest_a.reward_amount as f64 * success_factor) as u64);
                points_b = points_b.saturating_add(quest_b.reward_amount.saturating_sub(10));
                msg!("Quest A is harder, giving it bonus points.");
            } else {
                points_b = points_b.saturating_add((quest_b.reward_amount as f64 * success_factor) as u64);
                points_a = points_a.saturating_add(quest_a.reward_amount.saturating_sub(10));
                msg!("Quest B is harder, giving it bonus points.");
            }
        }

        // Apply reward logic
        if score_sum > 200 {
            quest_a.is_completed = true;
            quest_b.is_completed = true;
            
            // This is the problematic part. If quest_a and quest_b are the same, the reward is applied twice
            quest_a.reward_amount = quest_a.reward_amount.saturating_add(points_a);
            quest_b.reward_amount = quest_b.reward_amount.saturating_add(points_b);
            
            msg!("Both quests completed! Final rewards: A: {}, B: {}", quest_a.reward_amount, quest_b.reward_amount);
        } else {
            msg!("Scores too low, neither quest completed.");
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitQuestBoard<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 32 + 4 + 1)]
    pub board: Account<'info, QuestBoard>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitQuest<'info> {
    #[account(mut, has_one = quest_board)]
    pub quest_board: Account<'info, QuestBoard>,
    #[account(init, payer = admin, space = 8 + 32 + 4 + 1 + 8 + 1)]
    pub quest: Account<'info, Quest>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CompleteQuests<'info> {
    #[account(mut)]
    pub board: Account<'info, QuestBoard>,
    #[account(mut, has_one = quest_board)]
    pub quest_a: Account<'info, Quest>,
    #[account(mut, has_one = quest_board)]
    pub quest_b: Account<'info, Quest>,
}

#[account]
pub struct QuestBoard {
    pub admin: Pubkey,
    pub name: String,
    pub total_quests: u32,
    pub is_open: bool,
}

#[account]
pub struct Quest {
    pub quest_board: Pubkey,
    pub quest_id: u32,
    pub difficulty: u8,
    pub reward_amount: u64,
    pub is_completed: bool,
}

#[error_code]
pub enum QuestError {
    #[msg("Quest board is closed.")]
    BoardClosed,
}