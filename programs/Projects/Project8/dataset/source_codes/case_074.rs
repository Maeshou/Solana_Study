// 3) rating_adjuster: 最近成績から調整（軽い分岐あり）
use anchor_lang::prelude::*;

declare_id!("AdjR4te33333333333333333333333333333333");

#[program]
pub mod rating_adjuster {
    use super::*;

    pub fn adjust(ctx: Context<Adjust>, base_rating: u64, recent_scores: Vec<u32>, victories: Vec<bool>, draws: Vec<bool>) -> Result<()> {
        let n = recent_scores.len().min(20);
        let mut sum = 0u64;
        let mut win_count = 0u64;

        let mut i = 0usize;
        while i < n {
            if victories.get(i).copied().unwrap_or(false) {
                win_count = win_count.saturating_add(1);
                sum = sum.saturating_add(recent_scores[i] as u64);
            } else {
                if draws.get(i).copied().unwrap_or(false) {
                    sum = sum.saturating_add((recent_scores[i] as u64 * 3) / 4);
                } else {
                    sum = sum.saturating_add((recent_scores[i] as u64) / 2);
                }
            }
            i = i.saturating_add(1);
        }

        let denom = (n as u64).max(1);
        let weighted = (base_rating.saturating_mul(40) + (sum.saturating_mul(60) / denom)) / 100;
        ctx.accounts.rating_state.adjusted_rating = weighted;

        let mut trend = PerformanceTrend::Poor;
        let mut rate = 0u64;
        if n > 0 { rate = win_count.saturating_mul(100) / (n as u64); }
        if rate >= 80 { trend = PerformanceTrend::Excellent; }
        if rate < 80 && rate >= 65 { trend = PerformanceTrend::Strong; }
        if rate < 65 && rate >= 50 { trend = PerformanceTrend::Stable; }
        if rate < 50 && rate >= 35 { trend = PerformanceTrend::Declining; }
        ctx.accounts.rating_state.trend = trend;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Adjust<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + RatingState::LEN,
        seeds = [b"rating", user.key().as_ref()],
        bump
    )]
    pub rating_state: Account<'info, RatingState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RatingState {
    pub adjusted_rating: u64,
    pub trend: PerformanceTrend,
}
impl RatingState { pub const LEN: usize = 8 + 1; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum PerformanceTrend { Excellent, Strong, Stable, Declining, Poor }
