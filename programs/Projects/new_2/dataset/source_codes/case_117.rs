use anchor_lang::prelude::*;

declare_id!("RewardV8888888888888888888888888888888888");

#[program]
pub mod reward_claim_vuln {
    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        // reward.owner の検証なし
        let r = &mut ctx.accounts.reward;
        r.claimed = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub reward: Account<'info, RewardData>,
}

#[account]
pub struct RewardData {
    pub owner: Pubkey,
    pub claimed: bool,
}
