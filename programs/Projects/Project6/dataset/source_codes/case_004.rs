// ===============================================
// (4) palette_mixer: アート/色パレット（キャンバス・ブラシ・パレット）
// ===============================================
use anchor_lang::prelude::*;
declare_id!("Pa11eTeMix444444444444444444444444444444");

#[program]
pub mod palette_mixer {
    use super::*;

    pub fn init_canvas(ctx: Context<InitCanvas>, w: u16, h: u16) -> Result<()> {
        let c = &mut ctx.accounts.canvas;
        let p = &mut ctx.accounts.palette;

        c.owner = ctx.accounts.owner.key();
        c.size = (w as u32) * (h as u32);
        c.channel = (w ^ h) as u8;

        p.parent = c.key();
        p.mode = 0;
        p.ramps = [0u8; 8];
        Ok(())
    }

    pub fn init_brush(ctx: Context<InitBrush>, strength: u8) -> Result<()> {
        let b = &mut ctx.accounts.brush;
        b.parent = ctx.accounts.canvas.key();
        b.strength = strength;
        b.seed = 1;
        Ok(())
    }

    pub fn mix(ctx: Context<Mix>, bias: u8) -> Result<()> {
        let c = &mut ctx.accounts.canvas;
        let b = &mut ctx.accounts.brush;
        let p = &mut ctx.accounts.palette;

        for i in 0..p.ramps.len() {
            let v = ((p.ramps[i] as u16 + b.strength as u16 + bias as u16) & 0xFF) as u8;
            p.ramps[i] = v;
        }

        if (p.ramps[0] & 1) == 1 {
            b.seed = b.seed.rotate_left(1);
            c.size = c.size.saturating_add((p.ramps[0] as u32) * 3);
            p.mode = p.mode.saturating_add(1).min(200);
            msg!("odd path: seed rot, size up, mode step");
        } else {
            b.seed = b.seed.rotate_right(1);
            c.size = c.size.saturating_sub((p.ramps[0] as u32).min(c.size));
            p.mode = p.mode.saturating_sub(1);
            msg!("even path: seed rot back, size down, mode down");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitCanvas<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 1)]
    pub canvas: Account<'info, Canvas>,
    /// 固定パレット（例：PDA/定数）に pin
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 1 + 1*8,
        address = default_palette.key()
    )]
    pub palette: Account<'info, Palette>,
    /// CHECK: 固定先の公開鍵を受け渡すだけ
    pub default_palette: UncheckedAccount<'info>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitBrush<'info> {
    #[account(mut)]
    pub canvas: Account<'info, Canvas>,
    #[account(init, payer = owner, space = 8 + 32 + 1 + 1)]
    pub brush: Account<'info, Brush>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Mix<'info> {
    #[account(mut)]
    pub canvas: Account<'info, Canvas>,
    #[account(
        mut,
        has_one = parent
    )]
    pub brush: Account<'info, Brush>,
    #[account(
        mut,
        has_one = parent,
        constraint = brush.strength != palette.mode as u8 @ MixErr::Cosplay
    )]
    pub palette: Account<'info, Palette>,
}

#[account]
pub struct Canvas {
    pub owner: Pubkey,
    pub size: u32,
    pub channel: u8,
}

#[account]
pub struct Brush {
    pub parent: Pubkey, // = canvas
    pub strength: u8,
    pub seed: u8,
}

#[account]
pub struct Palette {
    pub parent: Pubkey, // = canvas
    pub mode: u8,
    pub ramps: [u8; 8],
}

#[error_code]
pub enum MixErr { #[msg("cosplay blocked")] Cosplay }
