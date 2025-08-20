use anchor_lang::prelude::*;

declare_id!("DailyCycle55555555555555555555555555555555");

#[program]
pub mod daily_cycle {
    use super::*;

    /// ログイン報酬受取
    pub fn claim(ctx: Context<ClaimDaily>, slot: u64) -> Result<u64> {
        let d = &mut ctx.accounts.daily;
        if slot > d.last_slot {
            d.last_slot = slot;
            let idx = (d.count % d.rewards.len() as u64) as usize;
            d.count = d.count.saturating_add(1);
            Ok(d.rewards[idx])
        } else {
            Ok(0)
        }
    }
}

#[derive(Accounts)]
pub struct ClaimDaily<'info> {
    #[account(mut)]
    pub daily: Account<'info, DailyCycleData>,
    pub user: Signer<'info>,
}

#[account]
pub struct DailyCycleData {
    pub last_slot: u64,
    pub count: u64,
    pub rewards: Vec<u64>,
}
