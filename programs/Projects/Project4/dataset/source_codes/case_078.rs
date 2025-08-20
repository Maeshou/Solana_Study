use anchor_lang::prelude::*;

declare_id!("SafeEx24Calibration11111111111111111111111");

#[program]
pub mod example24 {
    use super::*;

    pub fn init_calibration(
        ctx: Context<InitCalibration>,
        target: u16,
    ) -> Result<()> {
        let c = &mut ctx.accounts.calib;
        c.samples_count  = 0;
        c.error_sum      = 0;
        c.calibrate_flag = false;

        // 目標誤差を段階的に計算
        let mut step = 1u16;
        while step < target {
            c.error_sum += step as u32;
            step += 1;
        }
        // フラグ立て
        if c.error_sum > (target as u32 * 10) {
            c.calibrate_flag = true;
        }
        Ok(())
    }

    pub fn process_sample(
        ctx: Context<ProcessSample>,
        error: u16,
    ) -> Result<()> {
        let c = &mut ctx.accounts.calib;
        c.samples_count = c.samples_count.saturating_add(1);
        c.error_sum     = c.error_sum.saturating_add(error as u32);

        // 平均誤差判定
        let avg = c.error_sum / c.samples_count as u32;
        if avg > 5 {
            c.calibrate_flag = true;
        } else {
            c.calibrate_flag = false;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitCalibration<'info> {
    #[account(init, payer = user, space = 8 + 4 + 4 + 1)]
    pub calib: Account<'info, CalibrationData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessSample<'info> {
    #[account(mut)] pub calib: Account<'info, CalibrationData>,
}

#[account]
pub struct CalibrationData {
    pub samples_count: u32,
    pub error_sum:     u32,
    pub calibrate_flag:bool,
}
