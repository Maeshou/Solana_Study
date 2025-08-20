// 6. 分析＋キャッシュ管理
use anchor_lang::prelude::*;
declare_id!("ANLYAAAABBBBCCCCDDDDEEEEFFFF2222");

#[program]
pub mod misinit_analytics_v4 {
    use super::*;

    pub fn setup_analysis(ctx: Context<SetupAnalysis>, metric: f64) -> Result<()> {
        let an = &mut ctx.accounts.analytics;
        an.metric = metric;
        an.created_at = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn update_metric(ctx: Context<SetupAnalysis>, new_metric: f64) -> Result<()> {
        let an = &mut ctx.accounts.analytics;
        an.metric = new_metric;
        an.updated_at = Some(Clock::get()?.unix_timestamp);
        Ok(())
    }

    pub fn log_cache(ctx: Context<SetupAnalysis>, info: String) -> Result<()> {
        let ch = &mut ctx.accounts.cache_log;
        if ch.logs.len() >= 20 { ch.logs.remove(0); }
        ch.logs.push((info, Clock::get()?.unix_timestamp));
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetupAnalysis<'info> {
    #[account(init, payer = user, space = 8 + 8 + 8 + 1)] pub analytics: Account<'info, AnalyticsData>,
    #[account(mut)] pub cache_log: Account<'info, CacheLog>,
    #[account(mut)] pub user: Signer<'info>, pub system_program: Program<'info, System>,
}

#[account]
pub struct AnalyticsData { pub metric:f64, pub created_at:i64, pub updated_at:Option<i64> }
#[account]
pub struct CacheLog { pub logs: Vec<(String,i64)> }
