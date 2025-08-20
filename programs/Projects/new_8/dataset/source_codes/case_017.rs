use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("GaLlErYcNv00000000000000000000000000005");

#[program]
pub mod gallery_canvas {
    use super::*;

    pub fn add_canvas(ctx: Context<AddCanvas>, tag: [u8; 6], size: u16, bump: u8) -> Result<()> {
        let mut t = tag;
        for i in 0..t.len() {
            if !t[i].is_ascii_uppercase() { t[i] = b'A' + (i as u8 % 26); }
        }
        let mut s = size;
        if s < 12 { s = 12; }

        let seeds = [&ctx.accounts.artist.key().to_bytes()[..], &t[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump]).map_err(|_| error!(CnvErr::Cell))?;
        if addr != ctx.accounts.canvas_cell.key() { return Err(error!(CnvErr::Cell)); }

        let c = &mut ctx.accounts.canvas;
        c.artist = ctx.accounts.artist.key();
        c.tag = t;
        c.size = s;
        c.layers = c.layers.saturating_add(1);
        Ok(())
    }

    pub fn draw_stroke(ctx: Context<DrawStroke>, pixels: u32, bump: u8) -> Result<()> {
        let tag = ctx.accounts.canvas.tag;
        let seeds = [&ctx.accounts.artist.key().to_bytes()[..], &tag[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump]).map_err(|_| error!(CnvErr::Cell))?;
        if addr != ctx.accounts.canvas_cell.key() { return Err(error!(CnvErr::Cell)); }

        let c = &mut ctx.accounts.canvas;
        let mut px = pixels;
        if px > 200_000 { px = 200_000; }
        c.strokes = c.strokes.saturating_add(1);
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
pub enum CnvErr {
    #[msg("Canvas cell PDA mismatch")]
    Cell,
}
