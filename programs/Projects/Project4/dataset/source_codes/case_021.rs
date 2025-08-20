use anchor_lang::prelude::*;

declare_id!("Ex1000000000000000000000000000000000001");

#[program]
pub mod example1 {
    use super::*;

    // センサーを初回登録して読み取り合計を格納
    pub fn register_sensor(ctx: Context<RegisterSensor>, readings: Vec<u32>) -> Result<()> {
        let mut sum = 0;
        for &r in readings.iter() {
            if r > 0 { sum += r; }
        }
        let sensor = &mut ctx.accounts.sensor;       // ← initあり
        sensor.total = sum;
        Ok(())
    }

    // 累積値をリセット
    pub fn reset_sensor(ctx: Context<ResetSensor>) -> Result<()> {
        let sensor = &mut ctx.accounts.sensor;       // ← initなし：既存参照のみ
        sensor.total = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterSensor<'info> {
    #[account(init, payer = user, space = 8 + 4)]
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
}
