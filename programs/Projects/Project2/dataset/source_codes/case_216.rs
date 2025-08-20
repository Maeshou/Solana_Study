use anchor_lang::prelude::*;

declare_id!("HlthExa5555555555555555555555555555555555");

#[program]
pub mod health_extra {
    use super::*;

    pub fn apply(ctx: Context<ModifyHealth>, dmg: u64) -> Result<()> {
        let h = &mut ctx.accounts.health;
        if h.current_hp > dmg {
            // ダメージ適用
            h.current_hp -= dmg;
            h.damage_taken = h.damage_taken.saturating_add(dmg);
        } else {
            // 瀕死 → 蘇生トークン消費
            h.current_hp = h.max_hp / 2;
            if h.res_tokens > 0 {
                h.res_tokens -= 1;
                h.revived = true;
            } else {
                h.dead = true;
            }
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ModifyHealth<'info> {
    #[account(mut)]
    pub health: Account<'info, HealthExtraData>,
}

#[account]
pub struct HealthExtraData {
    pub max_hp: u64,
    pub current_hp: u64,
    pub damage_taken: u64,
    pub res_tokens: u8,
    pub revived: bool,
    pub dead: bool,
}
