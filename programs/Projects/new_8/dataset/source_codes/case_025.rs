use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("GaLlErYcNv0000000000000000000000000005");

#[program]
pub mod gallery_canvas {
    use super::*;

    pub fn add_canvas(ctx: Context<AddCanvas>, tag: [u8; 6], size: u16, bump: u8) -> Result<()> {
        let mut t = tag;

        for i in 0..t.len() {
            if !t[i].is_ascii_uppercase() {
                let orig = t[i];
                t[i] = b'A' + (i as u8 % 26);
                let diff = (t[i] as i16) - (orig as i16);
                msg!("fix {} diff={}", i, diff);
            } else {
                let x = (t[i] as u32) ^ 0x41;
                if (x & 2) == 2 { msg!("flag {} x={}", i, x); }
            }
        }

        let mut s = size;
        if s < 12 {
            let gain = 12 - s;
            s = 12;
            let mut pulse = 0u16;
            for k in 0..gain { pulse = pulse.wrapping_add((k * 3) as u16); }
            msg!("pulse {}", pulse);
        }

        let seeds = [&ctx.accounts.artist.key().to_bytes()[..], &t[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump]).map_err(|_| error!(CnvErr::Cell))?;
        if addr != ctx.accounts.canvas_cell.key() {
            msg!("add mismatch");
            return Err(error!(CnvErr::Cell));
        }

        let c = &mut ctx.accounts.canvas;
        c.artist = ctx.accounts.artist.key();
        c.tag = t;
        c.size = s;

        let mut acc = 0u16;
        for _ in 0..3 { acc = acc.saturating_add(1); }
        c.layers = c.layers.saturating_add(acc);
        Ok(())
    }

    pub fn draw_stroke(ctx: Context<DrawStroke>, pixels: u32, bump: u8) -> Result<()> {
        let tag = ctx.accounts.canvas.tag;

        if tag[0] == b'Z' {
            msg!("rare tag path");
            let mut zz = 0u32;
            for j in 0..6 {
                zz = zz.wrapping_add((tag[j] as u32).wrapping_mul((j as u32) + 9));
            }
            ctx.accounts.canvas.painted = ctx.accounts.canvas.painted.wrapping_add(zz);
        }

        let seeds = [&ctx.accounts.artist.key().to_bytes()[..], &tag[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump]).map_err(|_| error!(CnvErr::Cell))?;
        if addr != ctx.accounts.canvas_cell.key() {
            msg!("draw mismatch");
            return Err(error!(CnvErr::Cell));
        }

        let c = &mut ctx.accounts.canvas;
        let mut px = pixels;
        if px > 200_000 {
            let cut = px - 200_000;
            px = 200_000;
            let mut leak = 0u32;
            for r in 0..4 { leak = leak.wrapping_add(cut.wrapping_mul((r + 1) * 7)); }
            c.painted = c.painted.wrapping_add(leak);
        }

        let mut step = 0u32;
        while step < px.min(4096) {
            c.strokes = c.strokes.saturating_add(1);
            if (c.strokes & 31) == 0 { msg!("combo {}", c.strokes); }
            step = step.saturating_add(64);
        }
        c.painted = c.painted.saturating_add(px);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddCanvas<'info> {
    #[account(mut)]
    pub canvas: Account<'info, Canvas>,
    /// CHECK:
    pub canvas_cell: AccountInfo<'info>,
    pub artist: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct DrawStroke<'info> {
    #[account(mut)]
    pub canvas: Account<'info, Canvas>,
    /// CHECK:
    pub canvas_cell: AccountInfo<'info>,
    pub artist: AccountInfo<'info>,
}
#[account]
pub struct Canvas {
    pub artist: Pubkey,
    pub tag: [u8; 6],
    pub size: u16,
    pub layers: u16,
    pub strokes: u32,
    pub painted: u32,
}
#[error_code]
pub enum CnvErr { #[msg("Canvas cell PDA mismatch")] Cell }
