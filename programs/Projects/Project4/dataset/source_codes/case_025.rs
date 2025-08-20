use anchor_lang::prelude::*;

declare_id!("Ex1000000000000000000000000000000000001");

#[program]
pub mod example1 {
    use super::*;

    // センサーを初回登録して各種統計を計算
    pub fn register_sensor(ctx: Context<RegisterSensor>, readings: Vec<u32>) -> Result<()> {
        let mut sum = 0u32;
        let mut count = 0u32;
        let mut max = 0u32;
        for &r in readings.iter() {
            if r > 0 {
                sum += r;
                count += 1;
                if r > max {
                    max = r;
                }
            }
        }
        let avg = if count > 0 { sum / count } else { 0 };

        let sensor = &mut ctx.accounts.sensor;  // ← initあり
        sensor.total = sum;
        sensor.count = count;
        sensor.max = max;
        sensor.avg = avg;
        Ok(())
    }

    // 統計をリセットしてタイムスタンプを更新
    pub fn reset_sensor(ctx: Context<ResetSensor>) -> Result<()> {
        let sensor = &mut ctx.accounts.sensor;  // ← initなし：既存参照のみ
        sensor.total = 0;
        sensor.count = 0;
        sensor.max = 0;
        sensor.avg = 0;
        sensor.last_reset = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterSensor<'info> {
    #[account(init, payer = user, space = 8 + 4*4 + 8)]
    pub sensor: Account<'info, SensorData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ResetSensor<'info> {
    pub sensor: Account<'info, SensorData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct SensorData {
    pub total: u32,
    pub count: u32,
    pub max: u32,
    pub avg: u32,
    pub last_reset: i64,
}
