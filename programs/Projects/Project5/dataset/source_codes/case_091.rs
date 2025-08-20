// ======================================================================
// 4) Seismo Watch：地震観測（初期化＝グレイコードから初期振幅、順：子A→親→子B→テープ）
// ======================================================================
declare_id!("SEIS44444444444444444444444444444444444444");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum WatchMode { Quiet, Alert, Alarm }

#[program]
pub mod seismo_watch {
    use super::*;
    use WatchMode::*;

    pub fn init_station(ctx: Context<InitStation>, n: u32) -> Result<()> {
        let ga = (n ^ (n >> 1)) & 0xFFFF;
        let sa = &mut ctx.accounts.sensor_a;
        let st = &mut ctx.accounts.station;
        sa.station = st.key(); sa.channel = (n & 7) as u8; sa.amplitude = (ga % 701) + 21;

        st.owner = ctx.accounts.observer.key();
        st.threshold = n * 12 + 300;
        st.mode = Quiet;

        let sb = &mut ctx.accounts.sensor_b;
        sb.station = st.key(); sb.channel = ((n >> 2) & 7) as u8; sb.amplitude = ((ga.rotate_left(3)) % 709) + 19;

        let tp = &mut ctx.accounts.tape;
        tp.station = st.key(); tp.channel = 9; tp.ticks = 0; tp.hash = ga as u64 ^ 0xDEAD_BEEF;
        Ok(())
    }

    pub fn record(ctx: Context<RecordWave>, loops: u32) -> Result<()> {
        let st = &mut ctx.accounts.station;
        let a = &mut ctx.accounts.sensor_a;
        let b = &mut ctx.accounts.sensor_b;
        let tp = &mut ctx.accounts.tape;

        for i in 0..loops {
            let z = ((a.amplitude ^ b.amplitude) as u64).wrapping_mul(1469598103934665603);
            a.amplitude = a.amplitude.checked_add(((z & 63) as u32) + 3).unwrap_or(u32::MAX);
            b.amplitude = b.amplitude.saturating_add((((z >> 6) & 63) as u32) + 5);
            tp.ticks = tp.ticks.saturating_add(1);
            tp.hash ^= z.rotate_left((i % 19) as u32);
        }

        let sum = a.amplitude + b.amplitude;
        if sum > st.threshold {
            st.mode = Alarm;
            a.channel ^= 1; b.channel = b.channel.saturating_add(1);
            tp.channel = tp.channel.saturating_add(1);
            msg!("alarm: channel tweaks & tape move");
        } else {
            st.mode = Alert;
            a.amplitude = a.amplitude.saturating_add(9);
            b.amplitude = b.amplitude / 2 + 11;
            tp.hash ^= 0x0F0F_F0F0;
            msg!("alert: amp adjust & hash flip");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStation<'info> {
    #[account(init, payer=payer, space=8 + 32 + 4 + 1)]
    pub station: Account<'info, Station>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub sensor_a: Account<'info, Sensor>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub sensor_b: Account<'info, Sensor>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 8)]
    pub tape: Account<'info, Tape>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub observer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RecordWave<'info> {
    #[account(mut, has_one=owner)]
    pub station: Account<'info, Station>,
    #[account(
        mut,
        has_one=station,
        constraint = sensor_a.channel != sensor_b.channel @ SeisErr::Dup
    )]
    pub sensor_a: Account<'info, Sensor>,
    #[account(
        mut,
        has_one=station,
        constraint = sensor_b.channel != tape.channel @ SeisErr::Dup
    )]
    pub sensor_b: Account<'info, Sensor>,
    #[account(mut, has_one=station)]
    pub tape: Account<'info, Tape>,
    pub observer: Signer<'info>,
}

#[account] pub struct Station { pub owner: Pubkey, pub threshold: u32, pub mode: WatchMode }
#[account] pub struct Sensor  { pub station: Pubkey, pub channel: u8, pub amplitude: u32 }
#[account] pub struct Tape    { pub station: Pubkey, pub channel: u8, pub ticks: u64, pub hash: u64 }

#[error_code] pub enum SeisErr { #[msg("duplicate mutable account")] Dup }
