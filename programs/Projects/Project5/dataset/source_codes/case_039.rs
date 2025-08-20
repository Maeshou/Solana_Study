// ============================================================================
// 8) Totem Orchestra — Welford法で分散・平均を更新 — PDAなし
// ============================================================================
declare_id!("TOOR888888888888888888888888888888888");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum SessionState { Warmup, Playing, Finale }

#[program]
pub mod totem_orchestra {
    use super::*;

    pub fn init_orch(ctx: Context<InitOrch>, bound: u32) -> Result<()> {
        let x = &mut ctx.accounts;
        x.stage.conductor = x.conductor.key();
        x.stage.bound = bound;
        x.stage.state = SessionState::Warmup;
        Ok(())
    }

    pub fn perform(ctx: Context<Perform>, notes: u32) -> Result<()> {
        let x = &mut ctx.accounts;

        for k in 0..notes {
            // Welford: 平均と分散の1サンプル更新
            x.stats.count = x.stats.count.wrapping_add(1);
            let count = x.stats.count.max(1);
            let sample = (x.section.energy as u64) + (k as u64 * 7);
            let delta = sample as i128 - x.stats.mean as i128;
            x.stats.mean = (x.stats.mean as i128 + delta / count as i128) as u64;
            let delta2 = sample as i128 - x.stats.mean as i128;
            x.stats.m2 = (x.stats.m2 as i128 + (delta * delta2) / count as i128).max(0) as u64;

            // energyは非線形：E=E*3/2 + 5 (クリップ)
            let e = (u128::from(x.section.energy) * 3) / 2 + 5;
            x.section.energy = (e.min(u128::from(u32::MAX))) as u32;
        }

        if x.stats.mean > x.stage.bound as u64 {
            x.stage.state = SessionState::Finale;
            x.section.momentum = x.section.momentum.wrapping_add(3);
            x.stats.flags = x.stats.flags.wrapping_add(1);
            msg!("finale: momentum+3 flags+1");
        } else {
            x.stage.state = SessionState::Playing;
            x.section.energy = x.section.energy / 2 + 9;
            x.stats.count = x.stats.count.wrapping_add(2);
            msg!("playing: energy/2+9, count+2");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitOrch<'info> {
    #[account(init, payer=payer, space=8+32+4+1)]
    pub stage: Account<'info, Stage>,
    #[account(init, payer=payer, space=8+4+4)]
    pub section: Account<'info, Section>,
    #[account(init, payer=payer, space=8+8+8+4)]
    pub stats: Account<'info, StatPack>,
    #[account(mut)] pub payer: Signer<'info>,
    pub conductor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Perform<'info> {
    #[account(mut, has_one=conductor, constraint = stage.key() != section.key(), error = OrchErr::Dup)]
    pub stage: Account<'info, Stage>,
    #[account(mut, constraint = section.key() != stats.key(), error = OrchErr::Dup)]
    pub section: Account<'info, Section>,
    #[account(mut)]
    pub stats: Account<'info, StatPack>,
    pub conductor: Signer<'info>,
}

#[account] pub struct Stage { pub conductor: Pubkey, pub bound: u32, pub state: SessionState }
#[account] pub struct Section { pub energy: u32, pub momentum: u32 }
#[account] pub struct StatPack { pub count: u64, pub mean: u64, pub m2: u64, pub flags: u32 }

#[error_code] pub enum OrchErr { #[msg("dup")] Dup }
