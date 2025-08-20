use anchor_lang::prelude::*;

declare_id!("ScoreDecay9999999999999999999999999999999");

#[program]
pub mod score_decay {
    use super::*;

    pub fn update_score(ctx: Context<UpdateScore>, raw: u64) -> Result<()> {
        let s = &mut ctx.accounts.sdata;
        let decayed = raw.saturating_sub((s.decay_rate as u64).saturating_mul(s.tick_count));
        s.current = decayed;
        s.tick_count = s.tick_count.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateScore<'info> {
    #[account(mut)]
    pub sdata: Account<'info, ScoreData>,
}

#[account]
pub struct ScoreData {
    pub current: u64,
    pub decay_rate: u8,
    pub tick_count: u64,
}
