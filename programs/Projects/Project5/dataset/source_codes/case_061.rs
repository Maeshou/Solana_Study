// ============================================================================
// 6) Nebula Rally — レース（PDAなし / has_one + lane不一致）
// ============================================================================
declare_id!("NBRL66666666666666666666666666666666666666666");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum RallyStage { Grid, Sprint, Cooldown }

#[program]
pub mod nebula_rally {
    use super::*;
    use RallyStage::*;

    pub fn init_rally(ctx: Context<InitRally>, laps: u32) -> Result<()> {
        let r = &mut ctx.accounts;
        r.track.host = r.host.key();
        r.track.laps = laps;
        r.track.stage = Grid;

        r.cart_a.track = r.track.key(); r.cart_a.lane = 5;
        r.cart_b.track = r.track.key(); r.cart_b.lane = 8;
        r.stats.track = r.track.key(); r.stats.lane = 99;
        Ok(())
    }

    pub fn run(ctx: Context<Run>, tick: u32) -> Result<()> {
        let r = &mut ctx.accounts;

        for i in 0..tick {
            r.cart_a.time = r.cart_a.time.saturating_add(10 + (i % 3));
            r.cart_b.time = r.cart_b.time.saturating_add(9 + (i % 4));
            r.stats.total = r.stats.total.saturating_add(7);
        }

        if r.stats.total > (r.track.laps as u64) * 15 {
            r.track.stage = Cooldown;
            r.stats.heats = r.stats.heats.saturating_add(3);
            r.cart_b.flags = r.cart_b.flags.saturating_add(2);
            r.cart_a.flags = r.cart_a.flags.saturating_add(1);
            msg!("cooldown: heats+3, flags updated");
        } else {
            r.track.stage = Sprint;
            r.stats.heats = r.stats.heats.saturating_add(1);
            r.cart_a.time = r.cart_a.time.saturating_add(6);
            r.cart_b.time = r.cart_b.time.saturating_add(6);
            msg!("sprint: heats+1, times+6");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRally<'info> {
    #[account(init, payer=payer, space=8+32+4+1)]
    pub track: Account<'info, Track>,
    #[account(init, payer=payer, space=8+32+1+4+4)]
    pub cart_a: Account<'info, Cart>,
    #[account(init, payer=payer, space=8+32+1+4+4)]
    pub cart_b: Account<'info, Cart>,
    #[account(init, payer=payer, space=8+32+1+8+4)]
    pub stats: Account<'info, RallyStats>,
    #[account(mut)] pub payer: Signer<'info>,
    pub host: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Run<'info> {
    #[account(mut, has_one=host)]
    pub track: Account<'info, Track>,
    #[account(mut, has_one=track, constraint = cart_a.lane != cart_b.lane @ NrErr::Dup)]
    pub cart_a: Account<'info, Cart>,
    #[account(mut, has_one=track, constraint = cart_b.lane != stats.lane @ NrErr::Dup)]
    pub cart_b: Account<'info, Cart>,
    #[account(mut, has_one=track)]
    pub stats: Account<'info, RallyStats>,
    pub host: Signer<'info>,
}

#[account] pub struct Track { pub host: Pubkey, pub laps: u32, pub stage: RallyStage }
#[account] pub struct Cart { pub track: Pubkey, pub lane: u8, pub time: u32, pub flags: u32 }
#[account] pub struct RallyStats { pub track: Pubkey, pub lane: u8, pub total: u64, pub heats: u32 }
#[error_code] pub enum NrErr { #[msg("dup")] Dup }

