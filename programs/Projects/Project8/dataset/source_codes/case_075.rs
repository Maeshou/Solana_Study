// 4) performance_analyzer: 履歴→一貫性スコア行列（分岐とループをここに）
use anchor_lang::prelude::*;

declare_id!("PerfAna44444444444444444444444444444444");

#[program]
pub mod performance_analyzer {
    use super::*;

    pub fn analyze(ctx: Context<Analyze>, performance_scores: Vec<u32>, timestamps: Vec<i64>, opponent_ratings: Vec<u32>) -> Result<()> {
        let n = performance_scores.len().min(20);
        let mut rows: Vec<PerformanceRow> = Vec::new();

        let mut i = 0usize;
        while i < n {
            let win = performance_scores[i] > 70;
            let draw = performance_scores[i] >= 45 && performance_scores[i] <= 70;

            let window = 5usize.min(i + 1);
            let mut s = 0u32;
            let mut j = i + 1 - window;
            while j <= i {
                s = s.saturating_add(performance_scores[j]);
                if j == i { break; }
                j = j.saturating_add(1);
            }
            let avg = s / (window as u32);

            let mut acc = 0u32;
            let mut k = i + 1 - window;
            while k <= i {
                let d = (performance_scores[k] as i32 - avg as i32).pow(2) as u32;
                acc = acc.saturating_add(d);
                if k == i { break; }
                k = k.saturating_add(1);
            }
            let var = acc / (window as u32);
            let consistency = 100 - (var / 10).min(50);

            let mut trend = 0i32;
            if i + 1 >= 3 {
                let w = 10usize.min(i + 1);
                let half = w / 2;
                let start = i + 1 - w;
                let mid = i + 1 - half;
                let mut early = 0u32;
                let mut p = start;
                while p < mid {
                    early = early.saturating_add(performance_scores[p]);
                    p = p.saturating_add(1);
                }
                let mut late = 0u32;
                let mut q = mid;
                while q <= i {
                    late = late.saturating_add(performance_scores[q]);
                    if q == i { break; }
                    q = q.saturating_add(1);
                }
                trend = (late as i32 / (w - half) as i32) - (early as i32 / half as i32);
            }

            rows.push(PerformanceRow {
                ts: timestamps.get(i).copied().unwrap_or(0),
                opponent: opponent_ratings.get(i).copied().unwrap_or(0),
                score: performance_scores[i],
                win_flag: win,
                draw_flag: draw,
                consistency_score: consistency,
                improvement_trend: trend,
            });

            i = i.saturating_add(1);
        }

        ctx.accounts.analysis.rows = rows;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Analyze<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + AnalysisBook::LEN,
        seeds = [b"analysis", user.key().as_ref()],
        bump
    )]
    pub analysis: Account<'info, AnalysisBook>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct AnalysisBook {
    pub rows: Vec<PerformanceRow>,
}
impl AnalysisBook { pub const LEN: usize = 4 + 32 * PerformanceRow::LEN; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PerformanceRow {
    pub ts: i64,
    pub opponent: u32,
    pub score: u32,
    pub win_flag: bool,
    pub draw_flag: bool,
    pub consistency_score: u32,
    pub improvement_trend: i32,
}
impl PerformanceRow { pub const LEN: usize = 8 + 4 + 4 + 1 + 1 + 4 + 4; }
