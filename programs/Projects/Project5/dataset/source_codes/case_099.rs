// ======================================================================
// 2) Stained Glass Studio：ステンドグラス（PDAなし / has_one + 不一致）
//    - init_mosaic：親→パネルA→パネルB→ラック。ラジカルインバースで初期色
//    - assemble：分岐・ループなしで色とカウントを合成更新
// ======================================================================
declare_id!("GLAS2323232323232323232323232323232323232323");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum GlassPhase { Plan, Cut, Mount }

#[program]
pub mod stained_glass_studio {
    use super::*;
    use GlassPhase::*;

    fn radical_inverse_base2(mut x: u32) -> u32 {
        // 32bitの単純ビットリバース（低位重視の近似）
        let mut y = 0u32;
        for _ in 0..16 {
            y = (y << 1) | (x & 1);
            x >>= 1;
        }
        y
    }

    pub fn init_mosaic(ctx: Context<InitMosaic>, w: u32, h: u32) -> Result<()> {
        let s = &mut ctx.accounts.studio;
        s.owner = ctx.accounts.glazier.key();
        s.target = (w + h) * 6 + 240;
        s.phase = Plan;

        let code_a = radical_inverse_base2(w) ^ 0x5A5A;
        let code_b = radical_inverse_base2(h) ^ 0xA5A5;

        let a = &mut ctx.accounts.pane_a;
        a.parent = s.key();
        a.frame = (w & 7) as u8;
        a.hue = (code_a & 1023) + 41;

        let b = &mut ctx.accounts.pane_b;
        b.parent = s.key();
        b.frame = (h & 7) as u8;
        b.hue = (code_b.rotate_left(3) & 1023) + 37;

        let r = &mut ctx.accounts.rack;
        r.parent = s.key();
        r.frame = 9;
        r.count = 0;
        r.blend = ((code_a as u64) << 20) ^ (code_b as u64);

        Ok(())
    }

    pub fn assemble(ctx: Context<AssembleGlass>) -> Result<()> {
        let s = &mut ctx.accounts.studio;
        let a = &mut ctx.accounts.pane_a;
        let b = &mut ctx.accounts.pane_b;
        let r = &mut ctx.accounts.rack;

        let mix = ((a.hue ^ b.hue) as u64).wrapping_mul(1099511628211);
        a.hue = a.hue.checked_add(((mix & 63) as u32) + 5).unwrap_or(u32::MAX);
        b.hue = b.hue.saturating_add((((mix >> 6) & 63) as u32) + 7);
        r.count = r.count.saturating_add(3);
        r.blend ^= mix.rotate_left(11);
        s.phase = Cut;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMosaic<'info> {
    #[account(init, payer=payer, space=8 + 32 + 4 + 1)]
    pub studio: Account<'info, StudioGlass>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub pane_a: Account<'info, Pane>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub pane_b: Account<'info, Pane>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 8)]
    pub rack: Account<'info, Rack>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub glazier: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AssembleGlass<'info> {
    #[account(mut, has_one=owner)]
    pub studio: Account<'info, StudioGlass>,
    #[account(
        mut,
        has_one=studio,
        constraint = pane_a.frame != pane_b.frame @ GlassErr::Dup
    )]
    pub pane_a: Account<'info, Pane>,
    #[account(
        mut,
        has_one=studio,
        constraint = pane_b.frame != rack.frame @ GlassErr::Dup
    )]
    pub pane_b: Account<'info, Pane>,
    #[account(mut, has_one=studio)]
    pub rack: Account<'info, Rack>,
    pub glazier: Signer<'info>,
}

#[account]
pub struct StudioGlass {
    pub owner: Pubkey,
    pub target: u32,
    pub phase: GlassPhase,
}

#[account]
pub struct Pane {
    pub parent: Pubkey,
    pub frame: u8,   // 一意フィールド
    pub hue: u32,
}

#[account]
pub struct Rack {
    pub parent: Pubkey,
    pub frame: u8,   // 一意フィールド
    pub count: u64,
    pub blend: u64,
}

#[error_code]
pub enum GlassErr {
    #[msg("duplicate mutable account")]
    Dup,
}