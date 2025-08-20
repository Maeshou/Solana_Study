use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("RdCdln0111111111111111111111111111111111");

#[program]
pub mod daily_reward_cd {
    use super::*;

    pub fn claim_reward(ctx: Context<Claim>, slot: u64) -> Result<()> {
        let cd = &mut ctx.accounts.cooldown;
        // 最終取得スロットとの差を計算
        let delta = slot.saturating_sub(cd.last_claim_slot);
        if delta >= cd.interval_slots {
            cd.last_claim_slot = slot;
            cd.total_claims = cd.total_claims.saturating_add(1);
        } else {
            cd.denied_count = cd.denied_count.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub cooldown: Account<'info, RewardCooldown>,
    pub player: Signer<'info>,
}

#[account]
pub struct RewardCooldown {
    pub last_claim_slot: u64,
    pub interval_slots: u64,
    pub total_claims: u64,
    pub denied_count: u64,
}
