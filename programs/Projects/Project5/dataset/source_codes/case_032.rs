// ============================================================================
// 1) Map Painter — 領地の彩色（PDAあり: seeds + has_one + constraint + assert_ne!）
// ============================================================================
declare_id!("MPPN111111111111111111111111111111111");
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum PaletteMode { Draft, Blend, Seal }

#[program]
pub mod map_painter {
    use super::*;

    pub fn init_painter(ctx: Context<InitPainter>, hue_cap: u32) -> Result<()> {
        let game = &mut ctx.accounts;
        game.canvas.owner = game.artist.key();
        game.canvas.hue_cap = hue_cap;
        game.canvas.mode = PaletteMode::Blend;
        // palette/ledger の数値はゼロ初期化に任せる
        Ok(())
    }

    pub fn paint_step(ctx: Context<PaintStep>, strokes: u16) -> Result<()> {
        let game = &mut ctx.accounts;
        assert_ne!(game.palette.key(), game.region_a.key(), "palette/region_a must differ");
        assert_ne!(game.palette.key(), game.region_b.key(), "palette/region_b must differ");

        for i in 0..strokes {
            let w = 4 + (i % 5) as u32;
            game.region_a.saturation = game.region_a.saturation.saturating_add(w);
            game.region_b.saturation = game.region_b.saturation.saturating_add(w + 3);
            game.palette.spreads = game.palette.spreads.saturating_add(1);
        }

        let total = game.region_a.saturation.saturating_add(game.region_b.saturation);
        if total > game.canvas.hue_cap {
            game.canvas.mode = PaletteMode::Seal;
            game.ledger.awards = game.ledger.awards.saturating_add(2);
            game.palette.ink = game.palette.ink.saturating_add(9);
            msg!("cap exceeded: sealing; +awards, +ink");
        } else {
            game.canvas.mode = PaletteMode::Blend;
            game.ledger.ops = game.ledger.ops.saturating_add(1);
            game.palette.ink = game.palette.ink.saturating_add(3);
            msg!("within cap: blending; +ops, +ink");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPainter<'info> {
    #[account(init, payer=payer, space=8+32+4+1)]
    pub canvas: Account<'info, Canvas>,
    #[account(init, payer=payer, space=8+4+4)]
    pub region_a: Account<'info, Region>,
    #[account(init, payer=payer, space=8+4+4)]
    pub region_b: Account<'info, Region>,
    #[account(init, payer=payer, space=8+8+4, seeds=[b"palette", artist.key().as_ref()], bump)]
    pub palette: Account<'info, Palette>,
    #[account(init, payer=payer, space=8+8+8)]
    pub ledger: Account<'info, PaintLedger>,
    #[account(mut)] pub payer: Signer<'info>,
    pub artist: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PaintStep<'info> {
    #[account(mut, has_one=owner)]
    pub canvas: Account<'info, Canvas>,
    #[account(mut, constraint = region_a.key() != region_b.key(), error = PaintErr::Dup)]
    pub region_a: Account<'info, Region>,
    #[account(mut)]
    pub region_b: Account<'info, Region>,
    #[account(mut, seeds=[b"palette", owner.key().as_ref()], bump)]
    pub palette: Account<'info, Palette>,
    #[account(mut)]
    pub ledger: Account<'info, PaintLedger>,
    pub owner: Signer<'info>,
}

#[account] pub struct Canvas { pub owner: Pubkey, pub hue_cap: u32, pub mode: PaletteMode }
#[account] pub struct Region { pub saturation: u32, pub tiles: u32 }
#[account] pub struct Palette { pub ink: u32, pub spreads: u64 }
#[account] pub struct PaintLedger { pub ops: u64, pub awards: u64 }
#[error_code] pub enum PaintErr { #[msg("duplicate mutable account")] Dup }

