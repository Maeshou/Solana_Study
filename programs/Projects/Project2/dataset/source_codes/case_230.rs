use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("SkillUnk999999999999999999999999999999999");

#[program]
pub mod skill_unlock {
    use super::*;

    pub fn unlock(
        ctx: Context<Unlock>,
        skill_id: u8,
    ) -> Result<()> {
        let s = &mut ctx.accounts.skills;
        if s.prereq_met.get(&skill_id) == Some(&true) {
            // 条件達成：アンロック
            s.unlocked.insert(skill_id, true);
            s.unlock_count = s.unlock_count.saturating_add(1);
        } else {
            // 条件未達：拒否
            s.attempts = s.attempts.saturating_add(1);
            s.locked_attempts
                .entry(skill_id)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Unlock<'info> {
    #[account(mut)]
    pub skills: Account<'info, SkillData>,
}

#[account]
pub struct SkillData {
    pub prereq_met: BTreeMap<u8, bool>,
    pub unlocked: BTreeMap<u8, bool>,
    pub unlock_count: u64,
    pub attempts: u64,
    pub locked_attempts: BTreeMap<u8, u64>,
}
