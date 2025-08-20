use anchor_lang::prelude::*;

declare_id!("LdbResetV999999999999999999999999999999999");

#[program]
pub mod leaderboard_reset_vuln {
    pub fn reset(ctx: Context<Reset>) -> Result<()> {
        // lb.owner 検証なし
        let lb = &mut ctx.accounts.lb;
        lb.scores.clear();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Reset<'info> {
    #[account(mut)]
    pub lb: Account<'info, Leaderboard>,
}

#[account]
pub struct Leaderboard {
    pub owner: Pubkey,
    pub scores: Vec<(Pubkey, u64)>,
}
