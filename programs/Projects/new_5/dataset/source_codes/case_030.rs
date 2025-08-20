// ============================================================================
// 6) Palette Studio (two mutable palettes)
// ============================================================================
use anchor_lang::prelude::*;

declare_id!("ART66666666666666666666666666666666666666");

#[program]
pub mod palette_studio {
    use super::*;
    use ShadeMode::*;

    pub fn init_artist(ctx: Context<InitArtist>, seed: u32) -> Result<()> {
        let a = &mut ctx.accounts.artist;
        a.owner = ctx.accounts.owner.key();
        a.seed = seed;
        a.curve = 0;
        a.strokes = 0;
        Ok(())
    }

    pub fn init_palette(ctx: Context<InitPalette>, tag: u8) -> Result<()> {
        let p = &mut ctx.accounts.palette;
        p.parent = ctx.accounts.artist.key();
        p.tag = tag;
        p.mode = Linear;
        p.base = 128;
        p.range = 64;
        Ok(())
    }

    pub fn blend_two(ctx: Context<BlendTwo>, intensity: u32) -> Result<()> {
        let a = &mut ctx.accounts.artist;
        let p1 = &mut ctx.accounts.palette_a;
        let p2 = &mut ctx.accounts.palette_b;

        // simple running blend loop
        let mut tone: u32 = a.seed ^ intensity;
        for _ in 0..6 {
            tone = ((tone << 3) ^ (tone >> 2)) & 1023;
            a.strokes = a.strokes.saturating_add((tone & 7) as u64);
            a.curve = a.curve.saturating_add((tone % 19) as u32);
        }

        if p1.base + p1.range < 255 {
            p1.mode = Linear;
            p1.base = (p1.base + (intensity % 11) as u16).min(255);
            p1.range = (p1.range + (a.curve % 9) as u16).min(180);
            a.curve = a.curve / 2 + 3;
            msg!("P1 linear; base={}, range={}, curve={}", p1.base, p1.range, a.curve);
        } else {
            p1.mode = Curve;
            p1.base = p1.base.saturating_sub((intensity % 13) as u16);
            p1.range = (p1.range / 2).max(8);
            a.curve = a.curve.saturating_sub(5);
            msg!("P1 curve; base={}, range={}, curve={}", p1.base, p1.range, a.curve);
        }

        for _ in 0..3 {
            if p2.tag & 1 == 1 {
                p2.mode = Curve;
                p2.base = p2.base.saturating_add(((a.seed % 7) + 1) as u16);
                p2.range = p2.range.saturating_add(((a.curve % 5) + 2) as u16);
                msg!("P2 curve+; base={}, range={}", p2.base, p2.range);
            } else {
                p2.mode = Linear;
                p2.base = p2.base / 2 + 12;
                p2.range = p2.range.saturating_sub(3);
                msg!("P2 linear-; base={}, range={}", p2.base, p2.range);
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitArtist<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 4 + 8)]
    pub artist: Account<'info, Artist>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitPalette<'info> {
    #[account(mut)]
    pub artist: Account<'info, Artist>,
    #[account(init, payer = painter, space = 8 + 32 + 1 + 1 + 2 + 2)]
    pub palette: Account<'info, Palette>,
    #[account(mut)]
    pub painter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BlendTwo<'info> {
    #[account(mut)]
    pub artist: Account<'info, Artist>,
    #[account(mut, has_one = parent)]
    pub palette_a: Account<'info, Palette>,
    #[account(mut, has_one = parent)]
    pub palette_b: Account<'info, Palette>, // can alias
}

#[account]
pub struct Artist {
    pub owner: Pubkey,
    pub seed: u32,
    pub curve: u32,
    pub strokes: u64,
}

#[account]
pub struct Palette {
    pub parent: Pubkey,
    pub tag: u8,
    pub mode: ShadeMode,
    pub base: u16,
    pub range: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ShadeMode {
    Linear,
    Curve,
}
use ShadeMode::*;

#[error_code]
pub enum PaletteError {
    #[msg("blend failed")]
    BlendFailed,
}