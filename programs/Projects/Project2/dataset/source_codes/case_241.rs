use anchor_lang::prelude::*;

declare_id!("LBoard101010101010101010101010101010101010");

#[program]
pub mod leaderboard_update {
    use super::*;

    pub fn update_score(ctx: Context<UpdateScore>, score: u64) -> Result<()> {
        let lb = &mut ctx.accounts.board;
        lb.scores.push((ctx.accounts.user.key(), score));
        lb.scores.sort_by(|a, b| b.1.cmp(&a.1));
        if lb.scores.len() > 10 {
            lb.scores.pop();
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateScore<'info> {
    #[account(mut)]
    pub board: Account<'info, LeaderboardData>,
    pub user: Signer<'info>,
}

#[account]
pub struct LeaderboardData {
    pub scores: Vec<(Pubkey, u64)>,
}
