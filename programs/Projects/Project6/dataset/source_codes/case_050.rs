use anchor_lang::prelude::*;

declare_id!("QUEST555555555555555555555555555555555555555");

#[program]
pub mod quest_program {
    use super::*;
    /// 討伐クエストの進捗を更新する
    pub fn update_slay_quest_progress(ctx: Context<UpdateQuestProgress>) -> Result<()> {
        let quest_log = &mut ctx.accounts.player_quest_log;

        for objective in quest_log.objectives.iter_mut() {
            let goblin_monster_type_id = 3;
            let is_target_objective = (objective.objective_type == ObjectiveType::Slay) && (objective.target_id == goblin_monster_type_id);
            objective.progress = objective.progress.saturating_add(is_target_objective as u32);
        }
        
        msg!("Quest progress updated for slaying a monster.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateQuestProgress<'info> {
    #[account(mut, has_one = owner)]
    pub player_quest_log: Account<'info, PlayerQuestLog>,
    #[account(constraint = slain_monster.monster_type_id == 3 @ GameErrorCode::IncorrectMonsterType)]
    pub slain_monster: Account<'info, Monster>,
    pub owner: Signer<'info>,
}

#[account]
pub struct PlayerQuestLog {
    pub owner: Pubkey,
    pub objectives: Vec<QuestObjective>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct QuestObjective {
    pub objective_type: ObjectiveType,
    pub target_id: u32,
    pub progress: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ObjectiveType { Slay, Collect }

#[account]
pub struct Monster {
    pub monster_type_id: u32,
}

#[error_code]
pub enum GameErrorCode {
    #[msg("Incorrect monster type for this quest objective.")]
    IncorrectMonsterType,
}