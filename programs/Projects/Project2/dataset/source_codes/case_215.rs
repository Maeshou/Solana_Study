use anchor_lang::prelude::*;

declare_id!("EngExa4444444444444444444444444444444444");

#[program]
pub mod energy_extra {
    use super::*;

    pub fn update(ctx: Context<UpdateEnergy>, amount: u64, slot: u64) -> Result<()> {
        let e = &mut ctx.accounts.energy;
        let regain = slot.saturating_sub(e.last_slot) / e.interval;
        if regain > 0 {
            // 通常回復
            e.current = (e.current + regain).min(e.max);
            e.last_slot = slot;
            e.penalty = 0;
        } else {
            // 早すぎる使用 → ペナルティ
            e.penalty = e.penalty.saturating_add(amount);
            e.penalty_count = e.penalty_count.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateEnergy<'info> {
    #[account(mut)]
    pub energy: Account<'info, EnergyExtraData>,
}

#[account]
pub struct EnergyExtraData {
    pub max: u64,
    pub current: u64,
    pub interval: u64,
    pub last_slot: u64,
    pub penalty: u64,
    pub penalty_count: u64,
}
