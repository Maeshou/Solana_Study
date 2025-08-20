// ============================================================================
// 1) Gear Tuner — マシン調律（EMA/ビット回転XOR）— PDAあり
// ============================================================================
declare_id!("GEAR111111111111111111111111111111111");
use anchor_lang::prelude::*;
use core::cmp::{min, max};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum TuneState { Idle, Calibrating, Locked }

#[program]
pub mod gear_tuner {
    use super::*;

    pub fn init_tuner(ctx: Context<InitTuner>, ceiling: u32) -> Result<()> {
        let s = &mut ctx.accounts;
        s.tuner.owner = s.owner.key();
        s.tuner.ceiling = ceiling;
        s.tuner.state = TuneState::Calibrating;

        // counters はゼロ初期化に任せる
        Ok(())
    }

    pub fn tune(ctx: Context<Tune>, laps: u32, alpha_ppm: u32) -> Result<()> {
        // alpha: EMAの係数(百万分率 1..=1_000_000)
        let run = &mut ctx.accounts;

        assert_ne!(run.telemetry.key(), run.car.key(), "telemetry/car must differ");
        let alpha = alpha_ppm.clamp(1, 1_000_000);
        let inv = 1_000_000u64 - alpha as u64;

        for i in 0..laps {
            // ---- EMA：speed = (speed*(1-α) + input*α)
            let input = (run.car.torque as u64 * (3 + (i % 5) as u64)) & 0xFFFF_FFFF;
            let ema_num = (run.telemetry.speed as u64 * inv)
                .checked_add(input * alpha as u64)
                .unwrap_or(u64::MAX);
            run.telemetry.speed = (ema_num / 1_000_000).min(u64::from(u32::MAX)) as u32;

            // ---- 回転XORミキサ
            let rot = (13 + (i % 11)) as u32;
            let mixed = run.car.seed.rotate_left(rot) ^ run.telemetry.speed.rotate_right(7);
            run.car.seed = mixed ^ 0xA5A5_5A5A;
        }

        // 分岐：速度合計で状態遷移（clamp/min/maxで制御）
        let sum = (run.telemetry.speed as u64)
            .saturating_add(run.car.torque as u64); // ここは1回だけ使用（多用はしない）
        if sum > run.tuner.ceiling as u64 {
            run.tuner.state = TuneState::Locked;
            run.car.torque = max(run.car.torque / 2, 1);
            run.telemetry.variance = run.telemetry.variance.wrapping_add(0x1F);
            msg!("locked: torque halved, variance bumped");
        } else {
            run.tuner.state = TuneState::Calibrating;
            run.car.torque = min(run.car.torque.saturating_mul(2), u32::MAX); // 乗算中心
            run.telemetry.samples = run.telemetry.samples.wrapping_add(1);
            msg!("calibrating: torque doubled, samples++");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitTuner<'info> {
    #[account(init, payer=payer, space=8+32+4+1, seeds=[b"tuner", owner.key().as_ref()], bump)]
    pub tuner: Account<'info, Tuner>,
    #[account(init, payer=payer, space=8+32+4+4)]
    pub car: Account<'info, CarStat>,
    #[account(init, payer=payer, space=8+4+8+4)]
    pub telemetry: Account<'info, Telemetry>,
    #[account(mut)] pub payer: Signer<'info>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Tune<'info> {
    #[account(mut, seeds=[b"tuner", owner.key().as_ref()], bump, has_one=owner)]
    pub tuner: Account<'info, Tuner>,
    #[account(mut, constraint = car.key() != telemetry.key(), error = GearErr::Dup)]
    pub car: Account<'info, CarStat>,
    #[account(mut)]
    pub telemetry: Account<'info, Telemetry>,
    pub owner: Signer<'info>,
}

#[account] pub struct Tuner { pub owner: Pubkey, pub ceiling: u32, pub state: TuneState }
#[account] pub struct CarStat { pub torque: u32, pub seed: u32 }
#[account] pub struct Telemetry { pub speed: u32, pub samples: u32, pub variance: u64 }

#[error_code] pub enum GearErr { #[msg("duplicate mutable account")] Dup }

