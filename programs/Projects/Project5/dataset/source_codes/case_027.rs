// ============================================================================
// 3) Sky Circuit — 空中レース（PDAあり・ハッシュ混合・ローカル計算）
//    防止: seeds固定 + constraint（一対不一致） + assert_ne!
// ============================================================================
declare_id!("SKYC33333333333333333333333333333333");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum TrackState { Prep, Active, Cooldown }

#[program]
pub mod sky_circuit {
    use super::*;

    pub fn init_track(ctx: Context<InitTrack>, laps: u32) -> Result<()> {
        let accs = &mut ctx.accounts;
        accs.track.host = accs.host.key();
        accs.track.laps = laps;
        accs.track.state = TrackState::Active;
        Ok(())
    }

    pub fn run_heats(ctx: Context<RunHeats>, tick: u32) -> Result<()> {
        let a = &mut ctx.accounts;
        assert_ne!(a.track.key(), a.stats.key(), "track/stats must differ");

        // まとめてローカル演算
        let seed = [&a.track.host.as_ref(), &a.stats.total.to_le_bytes()].concat();
        let bump = (hashv(&[&seed]).0[0] as u32 % 9) + 3; // 3..=11

        for _ in 0..tick {
            a.racer_a.lap = a.racer_a.lap.saturating_add(10 + bump);
            a.racer_b.lap = a.racer_b.lap.saturating_add(12 + bump);
            a.stats.total = a.stats.total.saturating_add(7 + bump as u64);
        }

        if a.stats.total > (a.track.laps as u64) * 20 {
            a.track.state = TrackState::Cooldown;
            a.stats.heats = a.stats.heats.saturating_add(3 + (bump / 2));
            a.racer_b.flags = a.racer_b.flags.saturating_add(2);
            msg!("cooldown: heats & flags raised");
        } else {
            a.track.state = TrackState::Active;
            a.stats.heats = a.stats.heats.saturating_add(1);
            a.racer_a.flags = a.racer_a.flags.saturating_add(1);
            msg!("active: keep racing, heats/flags tick");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitTrack<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 4 + 1)]
    pub track: Account<'info, Track>,
    #[account(init, payer = payer, space = 8 + 32 + 8 + 1)]
    pub racer_a: Account<'info, Racer>,
    #[account(init, payer = payer, space = 8 + 32 + 8 + 1)]
    pub racer_b: Account<'info, Racer>,
    #[account(init, payer = payer, space = 8 + 8 + 4, seeds=[b"stats", host.key().as_ref()], bump)]
    pub stats: Account<'info, RaceStats>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub host: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RunHeats<'info> {
    #[account(mut)]
    pub track: Account<'info, Track>,
    #[account(mut, constraint = racer_a.key() != racer_b.key(), error = SkyErr::Dup)]
    pub racer_a: Account<'info, Racer>,
    #[account(mut)]
    pub racer_b: Account<'info, Racer>,
    #[account(mut, seeds=[b"stats", host.key().as_ref()], bump)]
    pub stats: Account<'info, RaceStats>,
    pub host: Signer<'info>,
}

#[account] pub struct Track { pub host: Pubkey, pub laps: u32, pub state: TrackState }
#[account] pub struct Racer { pub pilot: Pubkey, pub lap: u32, pub flags: u8 }
#[account] pub struct RaceStats { pub total: u64, pub heats: u32 }

#[error_code] pub enum SkyErr { #[msg("dup")] Dup }
