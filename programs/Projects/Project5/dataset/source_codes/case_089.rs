// ======================================================================
// 2) Airship Yard：飛行船ヤード（初期化＝popcountを用いた初期揚力、順序：子→子→親→索）
// ======================================================================
declare_id!("YARD222222222222222222222222222222222222222");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum YardPhase { Dock, Load, Hold }

#[program]
pub mod airship_yard {
    use super::*;
    use YardPhase::*;

    pub fn init_yard(ctx: Context<InitYard>, mass: u32) -> Result<()> {
        let a = &mut ctx.accounts.balloon_a;
        let b = &mut ctx.accounts.balloon_b;
        let y = &mut ctx.accounts.yard;

        // まず子を生成（親参照は後で確定し直す）
        a.yard = y.key(); a.mast = (mass & 7) as u8; a.lift = (mass.count_ones() * 7 + 50) as u32;
        b.yard = y.key(); b.mast = ((mass >> 3) & 7) as u8; b.lift = ((mass ^ (mass << 1)).count_ones() * 5 + 60) as u32;

        y.owner = ctx.accounts.marshal.key();
        y.capacity = mass.saturating_mul(3) + 400;
        y.phase = Dock;

        let t = &mut ctx.accounts.tether;
        t.yard = y.key(); t.mast = 9; t.strain = 0; t.knot = mass ^ 0x39A7;

        Ok(())
    }

    pub fn ballast(ctx: Context<Ballast>, steps: u32) -> Result<()> {
        let y = &mut ctx.accounts.yard;
        let a = &mut ctx.accounts.balloon_a;
        let b = &mut ctx.accounts.balloon_b;
        let t = &mut ctx.accounts.tether;

        for i in 0..steps {
            let mix = ((a.lift ^ b.lift) as u64).wrapping_mul(11400714819323198485);
            a.lift = a.lift.checked_add(((mix & 31) as u32) + 3).unwrap_or(u32::MAX);
            b.lift = b.lift.saturating_add((((mix >> 5) & 31) as u32) + 2);
            t.strain = t.strain.saturating_add((a.lift as u64 + b.lift as u64) & 0x7F);
            t.knot ^= (mix as u32).rotate_left((i % 13) as u32);
        }

        let total = a.lift + b.lift;
        if total > y.capacity {
            y.phase = Hold;
            a.mast ^= 1; b.mast = b.mast.saturating_add(1);
            t.mast = t.mast.saturating_add(1);
            msg!("hold: mast tweaks & tether move");
        } else {
            y.phase = Load;
            a.lift = a.lift.saturating_add(9);
            b.lift = b.lift / 2 + 11;
            t.knot ^= 0x0F0F_F0F0;
            msg!("load: lift adjust & knot flip");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitYard<'info> {
    #[account(init, payer=payer, space=8 + 32 + 4 + 1)]
    pub yard: Account<'info, YardCore>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub balloon_a: Account<'info, Balloon>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub balloon_b: Account<'info, Balloon>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 4)]
    pub tether: Account<'info, Tether>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub marshal: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Ballast<'info> {
    #[account(mut, has_one=owner)]
    pub yard: Account<'info, YardCore>,
    #[account(
        mut,
        has_one=yard,
        constraint = balloon_a.mast != balloon_b.mast @ YardErr::Dup
    )]
    pub balloon_a: Account<'info, Balloon>,
    #[account(
        mut,
        has_one=yard,
        constraint = balloon_b.mast != tether.mast @ YardErr::Dup
    )]
    pub balloon_b: Account<'info, Balloon>,
    #[account(mut, has_one=yard)]
    pub tether: Account<'info, Tether>,
    pub marshal: Signer<'info>,
}

#[account] pub struct YardCore { pub owner: Pubkey, pub capacity: u32, pub phase: YardPhase }
#[account] pub struct Balloon { pub yard: Pubkey, pub mast: u8, pub lift: u32 }
#[account] pub struct Tether  { pub yard: Pubkey, pub mast: u8, pub strain: u64, pub knot: u32 }

#[error_code] pub enum YardErr { #[msg("duplicate mutable account")] Dup }
