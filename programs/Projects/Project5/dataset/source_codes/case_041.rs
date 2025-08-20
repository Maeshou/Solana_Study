// 1) Flux Reactor — Q16.16で出力制御（PDAあり）
declare_id!("FLXR111111111111111111111111111111111");
use anchor_lang::prelude::*;
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum ReactorPhase { Warm, Stable, Over }
use ReactorPhase::*;

#[program]
pub mod flux_reactor {
    use super::*;

    pub fn init_reactor(ctx: Context<InitReactor>, cap_u16: u16) -> Result<()> {
        let s = &mut ctx.accounts;
        s.cfg.operator = s.operator.key();
        s.cfg.cap_q16 = (cap_u16 as u32) << 16; // Q16.16
        s.cfg.phase = Warm;
        Ok(())
    }

    pub fn step_reactor(ctx: Context<StepReactor>, ticks: u32, gain_q16: u32) -> Result<()> {
        let s = &mut ctx.accounts;
        assert_ne!(s.cfg.key(), s.core.key(), "cfg/core diff");
        let g = gain_q16.max(1); // >0

        for _ in 0..ticks {
            // power_q16 = power_q16 + power_q16 * g (Q16.16)
            let growth = ((s.core.power_q16 as u128) * (g as u128)) >> 16;
            let next = (s.core.power_q16 as u128) + growth;
            s.core.power_q16 = next.min(u128::from(u32::MAX)).try_into().unwrap_or(u32::MAX);

            // heat_q16 = heat_q16 + (g<<8) （Q16.16で微増）
            let bump = (g >> 8).max(1);
            s.core.heat_q16 = s.core.heat_q16.saturating_add(bump);
            s.gauge.cycles = s.gauge.cycles.wrapping_add(1);
        }

        if s.core.power_q16 > s.cfg.cap_q16 {
            s.cfg.phase = Over;
            s.gauge.trips = s.gauge.trips.wrapping_add(2);
            s.core.power_q16 = s.cfg.cap_q16;     // クリップ
            s.core.heat_q16 = s.core.heat_q16 >> 1; // 半減
            msg!("over cap: clip power, halve heat, +trips");
        } else {
            s.cfg.phase = Stable;
            s.gauge.stable = s.gauge.stable.wrapping_add(1);
            s.core.heat_q16 = s.core.heat_q16 + (1 << 12); // +0.000244140625
            s.core.power_q16 = s.core.power_q16 + (1 << 8); // 微増
            msg!("within cap: mark stable, tiny bumps");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitReactor<'info> {
    #[account(init, payer=payer, space=8+32+4+1, seeds=[b"cfg", operator.key().as_ref()], bump)]
    pub cfg: Account<'info, ReactorCfg>,
    #[account(init, payer=payer, space=8+4+4)]
    pub core: Account<'info, CoreQ16>,
    #[account(init, payer=payer, space=8+8+8)]
    pub gauge: Account<'info, ReactorGauge>,
    #[account(mut)] pub payer: Signer<'info>,
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct StepReactor<'info> {
    #[account(mut, seeds=[b"cfg", operator.key().as_ref()], bump, has_one=operator)]
    pub cfg: Account<'info, ReactorCfg>,
    #[account(mut, constraint = cfg.key() != core.key(), error = RxErr::Dup)]
    pub core: Account<'info, CoreQ16>,
    #[account(mut)]
    pub gauge: Account<'info, ReactorGauge>,
    pub operator: Signer<'info>,
}
#[account] pub struct ReactorCfg { pub operator: Pubkey, pub cap_q16: u32, pub phase: ReactorPhase }
#[account] pub struct CoreQ16 { pub power_q16: u32, pub heat_q16: u32 }
#[account] pub struct ReactorGauge { pub cycles: u64, pub trips: u32, pub stable: u32 }
#[error_code] pub enum RxErr { #[msg("dup")] Dup }
