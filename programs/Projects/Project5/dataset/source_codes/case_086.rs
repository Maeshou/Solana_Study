// ======================================================================
// 9) Snow Resort：リフト（初期化＝CRC風タグ）
// ======================================================================
declare_id!("SNOW99999999999999999999999999999999999999");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum LiftState { Prep, Run, Stop }

#[program]
pub mod snow_resort {
    use super::*;
    use LiftState::*;

    pub fn init_resort(ctx: Context<InitResort>, n: u32) -> Result<()> {
        let r = &mut ctx.accounts.resort;
        r.owner = ctx.accounts.manager.key();
        r.limit = n * 5 + 150;
        r.state = Prep;

        let a = &mut ctx.accounts.lift_a;
        let b = &mut ctx.accounts.lift_b;
        let c = &mut ctx.accounts.counter;

        // CRC風にざっくり
        let mut crc = 0xFFFF_FFFFu32 ^ n;
        for _ in 0..4 { let bit = (crc ^ (n >> 3)) & 1; crc = (crc >> 1) ^ (if bit==1 {0x1EDC6F41} else {0}); }

        a.resort = r.key(); a.gate = (n & 7) as u8; a.riders = (crc & 1023) + 30;
        b.resort = r.key(); b.gate = ((n >> 2) & 7) as u8; b.riders = ((crc >> 10) & 1023) + 27;

        c.resort = r.key(); c.gate = 9; c.laps = 0; c.seed = crc ^ 0xBEEF_BEEF;
        Ok(())
    }

    pub fn glide(ctx: Context<Glide>, loops: u32) -> Result<()> {
        let r = &mut ctx.accounts.resort;
        let a = &mut ctx.accounts.lift_a;
        let b = &mut ctx.accounts.lift_b;
        let c = &mut ctx.accounts.counter;

        for i in 0..loops {
            let z = ((a.riders ^ b.riders) as u64).wrapping_mul(1469598103934665603);
            a.riders = a.riders.checked_add(((z & 63) as u32) + 3).unwrap_or(u32::MAX);
            b.riders = b.riders.saturating_add((((z >> 6) & 63) as u32) + 5);
            c.laps = c.laps.saturating_add(1);
            c.seed ^= (z as u32).rotate_left((i % 19) as u32);
        }

        let sum = a.riders + b.riders;
        if sum > r.limit {
            r.state = Stop;
            a.gate ^= 1; b.gate = b.gate.saturating_add(1);
            c.gate = c.gate.saturating_add(1);
            msg!("stop: gate tweaks & counter move");
        } else {
            r.state = Run;
            a.riders = a.riders.saturating_add(9);
            b.riders = b.riders / 2 + 11;
            c.seed ^= 0x0F0F_F0F0;
            msg!("run: adjust riders & seed flip");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitResort<'info> {
    #[account(init, payer=payer, space=8 + 32 + 4 + 1)]
    pub resort: Account<'info, Resort>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub lift_a: Account<'info, Lift>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub lift_b: Account<'info, Lift>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 4)]
    pub counter: Account<'info, LiftCounter>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Glide<'info> {
    #[account(mut, has_one=owner)]
    pub resort: Account<'info, Resort>,
    #[account(
        mut,
        has_one=resort,
        constraint = lift_a.gate != lift_b.gate @ SnowErr::Dup
    )]
    pub lift_a: Account<'info, Lift>,
    #[account(
        mut,
        has_one=resort,
        constraint = lift_b.gate != counter.gate @ SnowErr::Dup
    )]
    pub lift_b: Account<'info, Lift>,
    #[account(mut, has_one=resort)]
    pub counter: Account<'info, LiftCounter>,
    pub manager: Signer<'info>,
}

#[account] pub struct Resort { pub owner: Pubkey, pub limit: u32, pub state: LiftState }
#[account] pub struct Lift   { pub resort: Pubkey, pub gate: u8, pub riders: u32 }
#[account] pub struct LiftCounter { pub resort: Pubkey, pub gate: u8, pub laps: u64, pub seed: u32 }

#[error_code] pub enum SnowErr { #[msg("duplicate mutable account")] Dup }
