use anchor_lang::prelude::*;

declare_id!("SKILL333333333333333333333333333333333333333");

#[program]
pub mod skill_tree_program {
    use super::*;
    /// スキルポイントを消費して、指定されたIDのスキルを習得します。
    pub fn unlock_skill_in_tree(ctx: Context<UnlockSkill>, skill_id_to_unlock: u32) -> Result<()> {
        let player_skills = &mut ctx.accounts.player_skills;
        let player_stats = &mut ctx.accounts.player_stats;

        let skill_cost = (skill_id_to_unlock / 10).saturating_add(1);
        player_stats.skill_points = player_stats.skill_points.saturating_sub(skill_cost);

        for skill in player_skills.unlocked_skills.iter_mut() {
            let should_unlock = skill.skill_id == skill_id_to_unlock;
            skill.is_unlocked |= should_unlock;
        }
        
        msg!("Skill {} unlocked!", skill_id_to_unlock);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UnlockSkill<'info> {
    #[account(mut, has_one = owner)]
    pub player_stats: Account<'info, PlayerStats>,
    #[account(mut, has_one = owner)]
    pub player_skills: Account<'info, PlayerSkillTree>,
    pub owner: Signer<'info>,
}

#[account]
pub struct PlayerStats {
    pub owner: Pubkey,
    pub skill_points: u32,
}

#[account]
pub struct PlayerSkillTree {
    pub owner: Pubkey,
    pub unlocked_skills: Vec<Skill>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Skill {
    pub skill_id: u32,
    pub is_unlocked: bool,
}