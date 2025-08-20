use anchor_lang::prelude::*;

declare_id!("BncTrack49494949494949494949494949494949");

#[program]
pub mod bounce_tracker49 {
    use super::*;

    /// カウントを増加
    pub fn bump(ctx: Context<Hit>) -> Result<()> {
        let t = &mut ctx.accounts.tracker;
        t.count = t.count.checked_add(1).unwrap();
        Ok(())
    }

    /// カウントをリセット
    pub fn reset(ctx: Context<Hit>) -> Result<()> {
        ctx.accounts.tracker.count = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Hit<'info> {
    #[account(mut)]
    pub tracker: Account<'info, BounceData>,
}

#[account]
pub struct BounceData {
    pub count: u64,
}
