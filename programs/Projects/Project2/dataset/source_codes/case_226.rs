use anchor_lang::prelude::*;

declare_id!("AttrEnh5555555555555555555555555555555555");

#[program]
pub mod attribute_enhance {
    use super::*;

    pub fn enhance(
        ctx: Context<Enhance>,
        cost: u64,
    ) -> Result<()> {
        let p = &mut ctx.accounts.player;
        if p.points >= cost {
            // 強化成功
            p.points -= cost;
            p.strength = p.strength.saturating_add(1);
            p.enhance_count = p.enhance_count.saturating_add(1);
            p.health_bonus = p.health_bonus.saturating_add(5);
        } else {
            // 強化失敗：ペナルティ
            p.penalty_count = p.penalty_count.saturating_add(1);
            p.points = 0;
            p.debuff_level = p.debuff_level.saturating_add(1);
            p.fail_log = p.fail_log.saturating_add(cost - p.points);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Enhance<'info> {
    #[account(mut)]
    pub player: Account<'info, PlayerAttr>,
}

#[account]
pub struct PlayerAttr {
    pub points: u64,
    pub strength: u8,
    pub enhance_count: u64,
    pub health_bonus: u64,
    pub penalty_count: u64,
    pub debuff_level: u8,
    pub fail_log: u64,
}
