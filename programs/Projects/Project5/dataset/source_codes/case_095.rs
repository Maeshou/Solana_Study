// ======================================================================
// 8) Pixel Studio：スプライト工房（初期化＝ビットインターリーブで初期インク量）
// ======================================================================
declare_id!("PXL888888888888888888888888888888888888888");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum PixelState { Idle, Paint, Export }

#[program]
pub mod pixel_studio {
    use super::*;
    use PixelState::*;

    fn interleave(mut x: u32, mut y: u32) -> u32 {
        // 簡易的なビット交互（低位8bit程度）
        let mut z = 0u32;
        for i in 0..8 {
            z |= ((x >> i) & 1) << (2 * i);
            z |= ((y >> i) & 1) << (2 * i + 1);
        }
        z
    }

    pub fn init_studio(ctx: Context<InitStudio>, w: u32, h: u32) -> Result<()> {
        let s = &mut ctx.accounts.studio;
        s.owner = ctx.accounts.artist.key();
        s.cap = (w + h) * 10 + 300;
        s.state = Idle;

        let code = interleave(w, h);
        let la = &mut ctx.accounts.layer_a;
        la.studio = s.key(); la.plane = (w & 7) as u8; la.ink = (code & 1023) + 33;

        let lb = &mut ctx.accounts.layer_b;
        lb.studio = s.key(); lb.plane = (h & 7) as u8; lb.ink = ((code.rotate_left(5)) & 1023) + 37;

        let at = &mut ctx.accounts.atlas;
        at.studio = s.key(); at.plane = 9; at.frames = 0; at.crc = code ^ 0xA5A5_5A5A;
        Ok(())
    }

    pub fn paint(ctx: Context<PaintSprite>, iters: u32) -> Result<()> {
        let s = &mut ctx.accounts.studio;
        let a = &mut ctx.accounts.layer_a;
        let b = &mut ctx.accounts.layer_b;
        let at = &mut ctx.accounts.atlas;

        for i in 0..iters {
            let mix = ((a.ink ^ b.ink) as u64).wrapping_mul(780291637);
            a.ink = a.ink.checked_add(((mix & 31) as u32) + 2).unwrap_or(u32::MAX);
            b.ink = b.ink.saturating_add((((mix >> 5) & 31) as u32) + 3);
            at.frames = at.frames.saturating_add(1);
            at.crc ^= (mix as u32).rotate_left((i % 13) as u32);
        }

        let sum = a.ink + b.ink + (at.frames as u32 & 0x3FF);
        if sum > s.cap {
            s.state = Export;
            a.plane ^= 1; b.plane = b.plane.saturating_add(1);
            at.plane = at.plane.saturating_add(1);
            msg!("export: plane tweaks & atlas move");
        } else {
            s.state = Paint;
            a.ink = a.ink.saturating_add(9);
            b.ink = b.ink / 2 + 11;
            at.crc ^= 0x0F0F_F0F0;
            msg!("paint: ink adjust & crc flip");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStudio<'info> {
    #[account(init, payer=payer, space=8 + 32 + 4 + 1)]
    pub studio: Account<'info, Studio>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub layer_a: Account<'info, Layer>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub layer_b: Account<'info, Layer>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 4)]
    pub atlas: Account<'info, Atlas>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub artist: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PaintSprite<'info> {
    #[account(mut, has_one=owner)]
    pub studio: Account<'info, Studio>,
    #[account(
        mut,
        has_one=studio,
        constraint = layer_a.plane != layer_b.plane @ PixelErr::Dup
    )]
    pub layer_a: Account<'info, Layer>,
    #[account(
        mut,
        has_one=studio,
        constraint = layer_b.plane != atlas.plane @ PixelErr::Dup
    )]
    pub layer_b: Account<'info, Layer>,
    #[account(mut, has_one=studio)]
    pub atlas: Account<'info, Atlas>,
    pub artist: Signer<'info>,
}

#[account] pub struct Studio { pub owner: Pubkey, pub cap: u32, pub state: PixelState }
#[account] pub struct Layer  { pub studio: Pubkey, pub plane: u8, pub ink: u32 }
#[account] pub struct Atlas  { pub studio: Pubkey, pub plane: u8, pub frames: u64, pub crc: u32 }

#[error_code] pub enum PixelErr { #[msg("duplicate mutable account")] Dup }
