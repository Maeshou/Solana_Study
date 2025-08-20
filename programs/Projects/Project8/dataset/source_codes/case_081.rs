// 9) anti_cheat: 入力パターンの一貫性（分岐は軽め、ここに集約）
use anchor_lang::prelude::*;

declare_id!("Ant1Ch34t999999999999999999999999999999");

#[program]
pub mod anti_cheat {
    use super::*;

    pub fn scan(ctx: Context<Scan>, reaction_ms: u32, decision_times: Vec<u32>, notes: Vec<String>) -> Result<()> {
        let n = decision_times.len();
        let mut sum = 0u32;
        let mut i = 0usize;
        while i < n {
            sum = sum.saturating_add(decision_times[i]);
            i = i.saturating_add(1);
        }

        let mut consistency = 50u32;
        if n > 0 {
            let mean = sum / (n as u32);
            let mut acc = 0u32;
            let mut k = 0usize;
            while k < n {
                let d = (decision_times[k] as i32 - mean as i32).pow(2) as u32;
                acc = acc.saturating_add(d);
                k = k.saturating_add(1);
            }
            let var = acc / (n as u32);
            let pen = (var / 100).min(50);
            consistency = 100 - pen;
        }

        ctx.accounts.report.analysis = GameplayPattern {
            average_reaction_time: reaction_ms,
            decision_making_consistency: consistency,
            input_pattern_variance: reaction_ms / 10,
            suspicious_activity_flags: notes,
        };
        ctx.accounts.report.checked_at = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Scan<'info> {
    #[account(
        init,
        payer = auditor,
        space = 8 + AntiCheatReport::LEN,
        seeds = [b"anticheat", auditor.key().as_ref()],
        bump
    )]
    pub report: Account<'info, AntiCheatReport>,
    #[account(mut)]
    pub auditor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct AntiCheatReport {
    pub analysis: GameplayPattern,
    pub checked_at: i64,
}
impl AntiCheatReport { pub const LEN: usize = GameplayPattern::LEN + 8; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct GameplayPattern {
    pub average_reaction_time: u32,
    pub decision_making_consistency: u32,
    pub input_pattern_variance: u32,
    pub suspicious_activity_flags: Vec<String>,
}
impl GameplayPattern { pub const LEN: usize = 4 + 4 + 4 + (4 + 8 * 32); }
