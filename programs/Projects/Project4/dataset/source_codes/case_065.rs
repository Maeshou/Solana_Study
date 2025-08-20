use anchor_lang::prelude::*;

declare_id!("SafeEx10Metrics11111111111111111111111111111");

#[program]
pub mod example10 {
    use super::*;

    /// メトリックを初期化
    pub fn init_metrics(
        ctx: Context<InitMetrics>,
        hits:   u32,
        misses: u32,
    ) -> Result<()> {
        let m = &mut ctx.accounts.metrics;
        m.hits = hits;
        m.misses = misses;
        // 精度はパーセンテージ
        let total = hits + misses;
        m.accuracy = if total == 0 {
            0
        } else {
            (hits * 100 / total) as u8
        };
        Ok(())
    }

    /// 成否を記録
    pub fn record(
        ctx: Context<Record>,
        success: bool,
    ) -> Result<()> {
        let m = &mut ctx.accounts.metrics;
        if success {
            m.hits += 1;
        } else {
            m.misses += 1;
        }
        let total = m.hits + m.misses;
        m.accuracy = (m.hits * 100 / total) as u8;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMetrics<'info> {
    #[account(init, payer = user, space = 8 + 4 + 4 + 1)]
    pub metrics: Account<'info, MetricsData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Record<'info> {
    #[account(mut)] pub metrics: Account<'info, MetricsData>,
}

#[account]
pub struct MetricsData {
    pub hits:    u32,
    pub misses:  u32,
    pub accuracy:u8,  // 0–100
}
