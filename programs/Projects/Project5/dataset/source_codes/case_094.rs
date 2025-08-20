// ======================================================================
// 7) Library Bindery：製本所（初期化＝桁和・交互和で原価計算、順：親→子A→台帳→子B）
// ======================================================================
declare_id!("BIND77777777777777777777777777777777777777");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum BindPhase { Prep, Stitch, Pause }

#[program]
pub mod library_bindery {
    use super::*;
    use BindPhase::*;

    pub fn init_bindery(ctx: Context<InitBindery>, isbn_like: u64) -> Result<()> {
        let mut ds = 0u32; let mut alt = 0i32; let mut sign = 1i32;
        let mut x = isbn_like;
        while x > 0 {
            let d = (x % 10) as i32;
            ds = ds + d as u32;
            alt = alt + sign * d;
            sign = -sign;
            x /= 10;
        }

        let b = &mut ctx.accounts.bindery;
        b.owner = ctx.accounts.binder.key();
        b.budget = ds.saturating_mul(10) + alt.unsigned_abs();
        b.phase = Prep;

        let s = &mut ctx.accounts.stack_a;
        s.bindery = b.key(); s.tray = (ds as u8) & 7; s.leaves = ds.saturating_mul(3) + 50;

        let l = &mut ctx.accounts.ledger;
        l.bindery = b.key(); l.tray = 9; l.count = 0; l.sign = (alt.unsigned_abs() as u64) << 8 | ds as u64;

        let t = &mut ctx.accounts.stack_b;
        t.bindery = b.key(); t.tray = ((ds >> 3) as u8) & 7; t.leaves = (ds ^ (alt.unsigned_abs())) + 60;

        Ok(())
    }

    pub fn stitch(ctx: Context<Stitch>, steps: u32) -> Result<()> {
        let b = &mut ctx.accounts.bindery;
        let s = &mut ctx.accounts.stack_a;
        let t = &mut ctx.accounts.stack_b;
        let l = &mut ctx.accounts.ledger;

        for i in 0..steps {
            let mix = ((s.leaves ^ t.leaves) as u64).wrapping_mul(1099511628211);
            s.leaves = s.leaves.checked_add(((mix & 31) as u32) + 2).unwrap_or(u32::MAX);
            t.leaves = t.leaves.saturating_add((((mix >> 5) & 31) as u32) + 3);
            l.count = l.count.saturating_add(1);
            l.sign ^= mix.rotate_left((i % 13) as u32);
        }

        let sum = s.leaves + t.leaves;
        if sum > b.budget {
            b.phase = Pause;
            s.tray ^= 1; t.tray = t.tray.saturating_add(1);
            l.tray = l.tray.saturating_add(1);
            msg!("pause: tray tweaks & ledger move");
        } else {
            b.phase = Stitch;
            s.leaves = s.leaves.saturating_add(9);
            t.leaves = t.leaves / 2 + 11;
            l.sign ^= 0x0F0F_F0F0;
            msg!("stitch: leaf adjust & sign flip");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBindery<'info> {
    #[account(init, payer=payer, space=8 + 32 + 4 + 1)]
    pub bindery: Account<'info, Bindery>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub stack_a: Account<'info, PaperStack>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 8)]
    pub ledger: Account<'info, BindLedger>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub stack_b: Account<'info, PaperStack>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub binder: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Stitch<'info> {
    #[account(mut, has_one=owner)]
    pub bindery: Account<'info, Bindery>,
    #[account(
        mut,
        has_one=bindery,
        constraint = stack_a.tray != stack_b.tray @ BindErr::Dup
    )]
    pub stack_a: Account<'info, PaperStack>,
    #[account(
        mut,
        has_one=bindery,
        constraint = stack_b.tray != ledger.tray @ BindErr::Dup
    )]
    pub stack_b: Account<'info, PaperStack>,
    #[account(mut, has_one=bindery)]
    pub ledger: Account<'info, BindLedger>,
    pub binder: Signer<'info>,
}

#[account] pub struct Bindery { pub owner: Pubkey, pub budget: u32, pub phase: BindPhase }
#[account] pub struct PaperStack { pub bindery: Pubkey, pub tray: u8, pub leaves: u32 }
#[account] pub struct BindLedger { pub bindery: Pubkey, pub tray: u8, pub count: u64, pub sign: u64 }

#[error_code] pub enum BindErr { #[msg("duplicate mutable account")] Dup }
