// 6) Avatar Palette Hash — 受動色調（PDAあり）
declare_id!("APHH666666666666666666666666666666666");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum PaintMode { Mono, Gradient, Prism }

#[program]
pub mod avatar_palette_hash {
    use super::*;
    use PaintMode::*;

    pub fn init_studio(ctx: Context<InitStudio>, limit: u32) -> Result<()> {
        let p = &mut ctx.accounts;
        p.studio.owner = p.owner.key();
        p.studio.limit = limit;
        p.studio.mode = Gradient;
        Ok(())
    }

    pub fn tint(ctx: Context<Tint>, steps: u32) -> Result<()> {
        let p = &mut ctx.accounts;

        for i in 0..steps {
            let h = hashv(&[p.studio.owner.as_ref(), &p.avatar.hue.to_le_bytes(), &i.to_le_bytes()]);
            let dv = (u16::from_le_bytes([h.0[0], h.0[1]]) % 181 + 7) as u32;
            p.avatar.hue = p.avatar.hue.rotate_left((dv % 17) + 1);
            p.avatar.sat = p.avatar.sat.wrapping_add((dv & 0x7F) as u32);
            p.log.strokes = p.log.strokes.wrapping_add(1);
        }

        if p.avatar.sat > p.studio.limit {
            p.studio.mode = Mono;
            p.log.badges = p.log.badges.wrapping_add(3);
            p.avatar.hue ^= 0x00FF_00FF;
            p.avatar.sat = p.avatar.sat / 2 + 11;
            msg!("mono: badges+3, hue xor, sat half+11");
        } else {
            p.studio.mode = Prism;
            p.log.strokes = p.log.strokes.wrapping_mul(2);
            p.avatar.hue = p.avatar.hue.rotate_right(3);
            p.avatar.sat = p.avatar.sat + 13;
            msg!("prism: strokes*2, hue rot, sat+13");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStudio<'info> {
    #[account(init, payer=payer, space=8+32+4+1, seeds=[b"studio", owner.key().as_ref()], bump)]
    pub studio: Account<'info, StudioHash>,
    #[account(init, payer=payer, space=8+4+4)]
    pub avatar: Account<'info, AvatarHue>,
    #[account(init, payer=payer, space=8+8+4)]
    pub log: Account<'info, TintLog>,
    #[account(mut)] pub payer: Signer<'info>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Tint<'info> {
    #[account(mut, seeds=[b"studio", owner.key().as_ref()], bump, has_one=owner)]
    pub studio: Account<'info, StudioHash>,
    #[account(
        mut,
        constraint = avatar.key() != studio.key() @ AphErr::Dup,
        constraint = avatar.key() != log.key() @ AphErr::Dup
    )]
    pub avatar: Account<'info, AvatarHue>,
    #[account(
        mut,
        constraint = log.key() != studio.key() @ AphErr::Dup
    )]
    pub log: Account<'info, TintLog>,
    pub owner: Signer<'info>,
}
#[account] pub struct StudioHash { pub owner: Pubkey, pub limit: u32, pub mode: PaintMode }
#[account] pub struct AvatarHue { pub hue: u32, pub sat: u32 }
#[account] pub struct TintLog { pub strokes: u64, pub badges: u32 }
#[error_code] pub enum AphErr { #[msg("dup")] Dup }
