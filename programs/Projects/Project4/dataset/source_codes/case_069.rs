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
        m.hits    = hits;
        m.misses  = misses;

        // 成功率を計算
        let total = m.hits + m.misses;
        let mut accuracy: u8 = 0;

        if total != 0 {
            // 基本の率を算出
            let rate = m.hits * 100 / total;
            msg!("Computed raw rate: {}%", rate);

            // u8 に収まるようクリップ
            if rate > u8::MAX as u32 {
                accuracy = u8::MAX;
                msg!("Rate clipped to u8::MAX");
            } else {
                accuracy = rate as u8;
                msg!("Rate within u8 range");
            }

            // 失敗率を計算して警告フラグを立てる
            let failure_rate = 100u32.saturating_sub(rate);
            msg!("Failure rate: {}%", failure_rate);

            let mut warn_flag = false;
            if failure_rate > 50 {
                warn_flag = true;
                msg!("Warning flag set: failure_rate > 50%");
            } else {
                msg!("Failure rate acceptable");
            }

            // 警告が立っていれば更に精度を少し下げる
            if warn_flag {
                let penalty = 5u8;
                accuracy = accuracy.saturating_sub(penalty);
                msg!("Applied penalty of {} → new accuracy: {}", penalty, accuracy);
            }

            // 補助情報として平均値を計算・表示
            let average = total / 2;
            msg!("Total/2 midpoint: {}", average);
        } else {
            // レコードがない場合の初期設定
            msg!("No records available, setting defaults");
            // デバッグ用フラグ処理
            let mut default_flag = true;
            if default_flag {
                msg!("Default flag is true");
            }
            accuracy = 0;
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
            msg!("Recorded a hit → hits={}", m.hits);
        } else {
            m.misses = m.misses.saturating_add(1);
            msg!("Recorded a miss → misses={}", m.misses);
        }

        // 成功率を再計算
        let total = m.hits + m.misses;
        let mut accuracy: u8 = 0;

        if total != 0 {
            let rate = m.hits * 100 / total;
            msg!("Recomputed raw rate: {}%", rate);

            if rate > u8::MAX as u32 {
                accuracy = u8::MAX;
                msg!("Rate clipped on record");
            } else {
                accuracy = rate as u8;
                msg!("Rate within range on record");
            }

            let failure_rate = 100u32.saturating_sub(rate);
            msg!("Recomputed failure rate: {}%", failure_rate);

            let mut warn_flag = false;
            if failure_rate > 50 {
                warn_flag = true;
                msg!("Warning flag re-set: failure_rate > 50%");
            } else {
                msg!("Failure rate still acceptable");
            }

            if warn_flag {
                let penalty = 3u8;
                accuracy = accuracy.saturating_sub(penalty);
                msg!("Applied record penalty of {} → accuracy={}", penalty, accuracy);
            }

            let average = total / 2;
            msg!("Record midpoint: {}", average);
        } else {
            msg!("After record: no data present");
            let mut default_flag = true;
            if default_flag {
                msg!("Default flag still true after record");
            }
            accuracy = 0;
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
    pub hits:     u32,
    pub misses:   u32,
    pub accuracy: u8,  // 0–100
}
