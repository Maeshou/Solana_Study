use anchor_lang::prelude::*;

declare_id!("SafeEx29DevHealth111111111111111111111111111");

#[program]
pub mod example29 {
    use super::*;

    pub fn init_health(
        ctx: Context<InitHealth>,
        temp_sum: u32,
        readings: u32,
    ) -> Result<()> {
        let d = &mut ctx.accounts.health;
        d.temp_sum    = temp_sum;
        d.readings    = readings;
        d.overheat    = false;

        // 平均温度判定
        let avg = d.temp_sum / (d.readings.max(1));
        if avg > 75 {
            d.overheat = true;
        }
        Ok(())
    }

    pub fn add_reading(
        ctx: Context<AddReading>,
        temp: u32,
    ) -> Result<()> {
        let d = &mut ctx.accounts.health;
        d.temp_sum = d.temp_sum.saturating_add(temp);
        d.readings = d.readings.saturating_add(1);

        let avg = d.temp_sum / d.readings;
        if avg > 80 {
            d.overheat = true;
        } else {
            d.overheat = false;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitHealth<'info> {
    #[account(init, payer = user, space = 8 + 4 + 4 + 1)]
    pub health: Account<'info, DeviceHealthData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddReading<'info> {
    #[account(mut)] pub health: Account<'info, DeviceHealthData>,
}

#[account]
pub struct DeviceHealthData {
    pub temp_sum: u32,
    pub readings: u32,
    pub overheat: bool,
}
