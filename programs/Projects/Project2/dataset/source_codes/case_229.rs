use anchor_lang::prelude::*;

declare_id!("PetTame8888888888888888888888888888888888");

#[program]
pub mod pet_tamer {
    use super::*;

    pub fn train(
        ctx: Context<Train>,
        gain: u64,
    ) -> Result<()> {
        let p = &mut ctx.accounts.pet;
        let req = p.level.saturating_mul(100);
        if p.experience + gain >= req {
            // レベルアップ
            p.experience = (p.experience + gain).saturating_sub(req);
            p.level = p.level.saturating_add(1);
            p.skill_points = p.skill_points.saturating_add(2);
        } else {
            // 失敗トレーニング
            p.failures = p.failures.saturating_add(1);
            p.experience = 0;
            p.fatigue_count = p.fatigue_count.saturating_add(1);
            p.rest_required = true;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Train<'info> {
    #[account(mut)]
    pub pet: Account<'info, PetData>,
}

#[account]
pub struct PetData {
    pub level: u64,
    pub experience: u64,
    pub skill_points: u64,
    pub failures: u64,
    pub fatigue_count: u64,
    pub rest_required: bool,
}
