// ======================================================================
// 9) Glacier Mapper：氷河調査（初期化＝Bresenham風ステップ合成）
// ======================================================================
declare_id!("GLCR99999999999999999999999999999999999999");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum IcePhase { Scout, Sample, Retreat }

#[program]
pub mod glacier_mapper {
    use super::*;
    use IcePhase::*;

    pub fn init_survey(ctx: Context<InitSurvey>, dx: u32, dy: u32) -> Result<()> {
        let s = &mut ctx.accounts.survey;
        s.owner = ctx.accounts.ranger.key();
        s.limit = dx + dy + 400;
        s.phase = Scout;

        // Bresenham-ish error accumulation
        let mut x = 0u32; let mut y = 0u32; let mut err: i32 = dx as i32 - dy as i32;
        let mut acc = 0u32;
        for _ in 0..8 {
            let e2 = 2 * err;
            if e2 > -(dy as i32) { err -= dy as i32; x += 1; }
            else { acc ^= 0x55AA; }
            if e2 < dx as i32 { err += dx as i32; y += 1; }
            else { acc ^= 0xAA55; }
        }

        let a = &mut ctx.accounts.ridge_a;
        a.survey = s.key(); a.band = (dx & 7) as u8; a.mass = (x + acc) + 21;

        let b = &mut ctx.accounts.ridge_b;
        b.survey = s.key(); b.band = (dy & 7) as u8; b.mass = (y + acc.rotate_left(3)) + 19;

        let lg = &mut ctx.accounts.log;
        lg.survey = s.key(); lg.band = 9; lg.steps = 0; lg.sig = ((x as u64) << 32) ^ y as u64 ^ acc as u64;

        Ok(())
    }

    pub fn traverse(ctx: Context<Traverse>, n: u32) -> Result<()> {
        let s = &mut ctx.accounts.survey;
        let a = &mut ctx.accounts.ridge_a;
        let b = &mut ctx.accounts.ridge_b;
        let lg = &mut ctx.accounts.log;

        for i in 0..n {
            let z = ((a.mass ^ b.mass) as u64).wrapping_mul(1469598103934665603);
            a.mass = a.mass.checked_add(((z & 63) as u32) + 3).unwrap_or(u32::MAX);
            b.mass = b.mass.saturating_add((((z >> 6) & 63) as u32) + 5);
            lg.steps = lg.steps.saturating_add(1);
            lg.sig ^= z.rotate_left((i % 19) as u32);
        }

        let sum = a.mass + b.mass;
        if sum > s.limit {
            s.phase = Retreat;
            a.band ^= 1; b.band = b.band.saturating_add(1);
            lg.band = lg.band.saturating_add(1);
            msg!("retreat: band tweaks & log move");
        } else {
            s.phase = Sample;
            a.mass = a.mass.saturating_add(9);
            b.mass = b.mass / 2 + 11;
            lg.sig ^= 0x0F0F_F0F0;
            msg!("sample: mass adjust & sig flip");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitSurvey<'info> {
    #[account(init, payer=payer, space=8 + 32 + 4 + 1)]
    pub survey: Account<'info, Survey>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub ridge_a: Account<'info, Ridge>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub ridge_b: Account<'info, Ridge>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 8)]
    pub log: Account<'info, IceLog>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub ranger: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Traverse<'info> {
    #[account(mut, has_one=owner)]
    pub survey: Account<'info, Survey>,
    #[account(
        mut,
        has_one=survey,
        constraint = ridge_a.band != ridge_b.band @ IceErr::Dup
    )]
    pub ridge_a: Account<'info, Ridge>,
    #[account(
        mut,
        has_one=survey,
        constraint = ridge_b.band != log.band @ IceErr::Dup
    )]
    pub ridge_b: Account<'info, Ridge>,
    #[account(mut, has_one=survey)]
    pub log: Account<'info, IceLog>,
    pub ranger: Signer<'info>,
}

#[account] pub struct Survey { pub owner: Pubkey, pub limit: u32, pub phase: IcePhase }
#[account] pub struct Ridge  { pub survey: Pubkey, pub band: u8, pub mass: u32 }
#[account] pub struct IceLog { pub survey: Pubkey, pub band: u8, pub steps: u64, pub sig: u64 }

#[error_code] pub enum IceErr { #[msg("duplicate mutable account")] Dup }
