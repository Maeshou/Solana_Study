use anchor_lang::prelude::*;

declare_id!("NextCaseEx10101010101010101010101010101010");

#[program]
pub mod example1 {
    use super::*;

    // センサーを登録（sensor にだけ init）
    pub fn process_readings(ctx: Context<ProcessReadings>, readings: Vec<u32>) -> Result<()> {
        let mut sum = 0u32;
        for &r in readings.iter() {
            if r > 0 {
                sum += r;
            }
        }
        let sensor = &mut ctx.accounts.sensor;         // ← initあり
        sensor.last_sum = sum;

        let log = &mut ctx.accounts.reading_log;       // ← initなし（本来は初期化すべき）
        log.count = readings.len() as u32;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ProcessReadings<'info> {
    #[account(init, payer = user, space = 8 + 4)]
    pub sensor: Account<'info, SensorData>,
    pub reading_log: Account<'info, ReadingLog>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct SensorData {
    pub last_sum: u32,
}

#[account]
pub struct ReadingLog {
    pub count: u32,
}
