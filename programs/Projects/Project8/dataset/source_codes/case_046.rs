use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_game {
    use super::*;

    // 新しいスキルを習得する
    pub fn learn_new_skill(ctx: Context<LearnNewSkill>, skill_id_to_learn: u16) -> Result<()> {
        let character = &mut ctx.accounts.character_stats;
        let skill_tree = &ctx.accounts.skill_tree;
        
        // スキルツリーから習得したいスキルを探す
        let mut target_skill_option: Option<Skill> = None;
        for skill in skill_tree.skills.iter() {
            if skill.skill_id == skill_id_to_learn {
                target_skill_option = Some(skill.clone());
            }
        }
        let target_skill = target_skill_option.ok_or(GameError::SkillNotFoundInTree)?;

        // 既に習得済みでないかチェック
        for learned_skill in character.learned_skills.iter() {
            require!(*learned_skill != skill_id_to_learn, GameError::SkillAlreadyLearned);
        }

        // スキル習得条件のチェック
        require!(character.level >= target_skill.required_level, GameError::RequiredLevelNotMet);
        require!(character.skill_points > 0, GameError::NotEnoughSkillPoints);

        // 前提スキルを習得しているかチェック
        if target_skill.prerequisite_skill_id != 0 {
            let mut prerequisite_learned = false;
            for learned_skill in character.learned_skills.iter() {
                if *learned_skill == target_skill.prerequisite_skill_id {
                    prerequisite_learned = true;
                }
            }
            require!(prerequisite_learned, GameError::PrerequisiteSkillNotLearned);
        }

        // スキルポイントを消費してスキルを習得
        character.skill_points -= 1;
        character.learned_skills.push(skill_id_to_learn);

        msg!("Character learned a new skill: {}!", skill_id_to_learn);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(skill_id_to_learn: u16)]
pub struct LearnNewSkill<'info> {
    #[account(mut, seeds = [b"character_stats", player.key().as_ref()], bump = character_stats.bump)]
    pub character_stats: Account<'info, CharacterStats>,
    #[account(seeds = [b"skill_tree", skill_tree.class_id.to_le_bytes().as_ref()], bump = skill_tree.bump)]
    pub skill_tree: Account<'info, SkillTree>,
    #[account(mut)]
    pub player: Signer<'info>,
}

#[account]
pub struct CharacterStats {
    pub level: u16,
    pub skill_points: u8,
    pub learned_skills: Vec<u16>,
    pub bump: u8,
}

#[account]
pub struct SkillTree {
    pub class_id: u8,
    pub skills: Vec<Skill>,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Skill {
    pub skill_id: u16,
    pub required_level: u16,
    pub prerequisite_skill_id: u16,
}

#[error_code]
pub enum GameError {
    #[msg("The specified skill was not found in the skill tree.")]
    SkillNotFoundInTree,
    #[msg("You have already learned this skill.")]
    SkillAlreadyLearned,
    #[msg("Your level is not high enough to learn this skill.")]
    RequiredLevelNotMet,
    #[msg("You do not have enough skill points.")]
    NotEnoughSkillPoints,
    #[msg("You have not learned the prerequisite skill.")]
    PrerequisiteSkillNotLearned,
}