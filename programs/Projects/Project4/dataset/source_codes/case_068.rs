use anchor_lang::prelude::*;

declare_id!("SafeExMetrics11111111111111111111111111111");

#[program]
pub mod safe_metrics {
    use super::*;

    /// メトリックアカウントを初期化
    pub fn init_metrics(
        ctx: Context<InitMetrics>,
        hits: u32,
        misses: u32,
    ) -> Result<()> {
        let m = &mut ctx.accounts.metrics;
        m.hits = hits;
        m.misses = misses;

        // 成功率を計算
        let total = m.hits + m.misses;
        let mut accuracy: u8 = 0;
        if total != 0 {
            // 率を100分率で算出
            let rate = m.hits * 100 / total;
            // u8 に収まるようクリップ
            if rate > u8::MAX as u32 {
                accuracy = u8::MAX;
            } else {
                accuracy = rate as u8;
            }
        }
        m.accuracy = accuracy;

        Ok(())
    }

    /// 成否を記録して成功率を再計算
    pub fn record(
        ctx: Context<Record>,
        success: bool,
    ) -> Result<()> {
        let m = &mut ctx.accounts.metrics;

        // 成功／失敗カウント
        if success {
            m.hits = m.hits.saturating_add(1);
        } else {
            m.misses = m.misses.saturating_add(1);
        }

        // 成功率を再計算
        let total = m.hits + m.misses;
        let mut accuracy: u8 = 0;
        if total != 0 {
            let rate = m.hits * 100 / total;
            if rate > u8::MAX as u32 {
                accuracy = u8::MAX;
            } else {
                accuracy = rate as u8;
            }
        }
        m.accuracy = accuracy;

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
