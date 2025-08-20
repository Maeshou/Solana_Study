// 6. 分析＋キャッシュ管理（Clockなし）
use anchor_lang::prelude::*;
declare_id!("ANLYZZZZYYYYXXXXWWWWVVVVUUUUTTTT");

#[program]
pub mod misinit_analytics_no_clock {
    use super::*;

    pub fn setup_analysis(ctx: Context<SetupAnalysis>, metric: f64) -> Result<()> {
        let an = &mut ctx.accounts.analytics;
        an.metric = metric;
        an.version = 1;
        Ok(())
    }

    pub fn update_metric(ctx: Context<SetupAnalysis>, new_metric: f64) -> Result<()> {
        let an = &mut ctx.accounts.analytics;
        an.metric = new_metric;
        an.version = an.version.checked_add(1).unwrap();
        Ok(())
    }

    pub fn log_cache(ctx: Context<SetupAnalysis>, label: String) -> Result<()> {
        let ch = &mut ctx.accounts.cache_log;
        if ch.labels.len() >= 5 { ch.labels.remove(0); }
        ch.labels.push(label);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetupAnalysis<'info> {
    #[account(init, payer = user, space = 8 + 8 + 1)] pub analytics: Account<'info, AnalyticsData>,
    #[account(mut)] pub cache_log: Account<'info, CacheLog>,
    #[account(mut)] pub user: Signer<'info>, pub system_program: Program<'info, System>,
}

#[account]
pub struct AnalyticsData { pub metric:f64, pub version:u8 }
#[account]
pub struct CacheLog { pub labels: Vec<String> }

