use anchor_lang::prelude::*;

declare_id!("Engrgy1111111111111111111111111111111111");

#[program]
pub mod energy_tracker {
    use super::*;

    /// 初期化：最大エネルギーとリチャージ間隔を設定
    pub fn init(ctx: Context<InitEnergy>, max_energy: u64, refill_interval: u64) -> Result<()> {
        let e = &mut ctx.accounts.energy;
        e.max = max_energy;
        e.current = max_energy;
        e.interval = refill_interval;
        e.last_used_slot = 0;
        Ok(())
    }

    /// エネルギー消費
    pub fn spend(ctx: Context<ModifyEnergy>, amount: u64, current_slot: u64) -> Result<()> {
        let e = &mut ctx.accounts.energy;
        // 前回スロットからの差分だけ回復
        let passed = current_slot.saturating_sub(e.last_used_slot);
        let regain = passed / e.interval;
        e.current = (e.current + regain).min(e.max);
        // 消費
        if amount <= e.current {
            e.current -= amount;
            e.last_used_slot = current_slot;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitEnergy<'info> {
    #[account(init, payer = user, space = 8 + 8*4)]
    pub energy: Account<'info, EnergyData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyEnergy<'info> {
    #[account(mut)] pub energy: Account<'info, EnergyData>,
}

#[account]
pub struct EnergyData {
    pub max: u64,
    pub current: u64,
    pub interval: u64,
    pub last_used_slot: u64,
}
