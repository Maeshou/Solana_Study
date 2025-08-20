// ======================================================================
// 3) Bonsai Atelier：盆栽工房（初期化＝整数平方根で活力を割り当て、順：ログ→親→木A→木B）
// ======================================================================
declare_id!("BNSI33333333333333333333333333333333333333");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Season { Dormant, Prune, Grow }

#[program]
pub mod bonsai_atelier {
    use super::*;
    use Season::*;

    pub fn init_atelier(ctx: Context<InitAtelier>, water: u16) -> Result<()> {
        let j = &mut ctx.accounts.journal;
        j.atelier = ctx.accounts.atelier.key();
        j.twig = 9;
        j.notes = 0;
        j.checksum = 1;

        let a = &mut ctx.accounts.atelier;
        a.owner = ctx.accounts.mist.key();
        a.water = water;
        a.season = Dormant;

        // sqrt(water * 1000) で活力
        let prod = (water as u32).saturating_mul(1000).max(1);
        let mut x = prod as u64;
        let mut y = (x + 1) / 2;
        while y < x { x = y; y = (x + prod as u64 / x) / 2; }
        let base = x as u32;

        let t1 = &mut ctx.accounts.tree_a;
        t1.atelier = a.key(); t1.twig = (water as u8) & 7; t1.vigor = base + 7;

        let t2 = &mut ctx.accounts.tree_b;
        t2.atelier = a.key(); t2.twig = ((water >> 2) as u8) & 7; t2.vigor = base.rotate_left(3) + 11;

        j.atelier = a.key(); // 念のため正親
        Ok(())
    }

    pub fn tend(ctx: Context<TendTrees>, days: u32) -> Result<()> {
        let a = &mut ctx.accounts.atelier;
        let t1 = &mut ctx.accounts.tree_a;
        let t2 = &mut ctx.accounts.tree_b;
        let j = &mut ctx.accounts.journal;

        for i in 0..days {
            let swing = ((t1.vigor ^ t2.vigor) as u64).wrapping_mul(2654435761);
            t1.vigor = t1.vigor.checked_add(((swing & 31) as u32) + 2).unwrap_or(u32::MAX);
            t2.vigor = t2.vigor.saturating_add((((swing >> 5) & 31) as u32) + 3);
            j.notes = j.notes.saturating_add((t1.vigor as u64 + t2.vigor as u64) & 63);
            j.checksum ^= (swing as u32).rotate_left((i % 7) as u32);
        }

        let sum = t1.vigor + t2.vigor;
        if sum > (a.water as u32) * 20 {
            a.season = Prune;
            t1.twig ^= 1; t2.twig = t2.twig.saturating_add(1);
            j.twig = j.twig.saturating_add(1);
            msg!("prune: twig tweaks & journal move");
        } else {
            a.season = Grow;
            t1.vigor = t1.vigor.saturating_add(9);
            t2.vigor = t2.vigor / 2 + 11;
            j.checksum ^= 0x0F0F_F0F0;
            msg!("grow: vigor adjust & checksum flip");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitAtelier<'info> {
    #[account(init, payer=payer, space=8 + 32 + 2 + 1)]
    pub atelier: Account<'info, Atelier>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub tree_a: Account<'info, TreeSpec>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 4)]
    pub tree_b: Account<'info, TreeSpec>,
    #[account(init, payer=payer, space=8 + 32 + 1 + 8 + 4)]
    pub journal: Account<'info, Journal>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub mist: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TendTrees<'info> {
    #[account(mut, has_one=owner)]
    pub atelier: Account<'info, Atelier>,
    #[account(
        mut,
        has_one=atelier,
        constraint = tree_a.twig != tree_b.twig @ BonsaiErr::Dup
    )]
    pub tree_a: Account<'info, TreeSpec>,
    #[account(
        mut,
        has_one=atelier,
        constraint = tree_b.twig != journal.twig @ BonsaiErr::Dup
    )]
    pub tree_b: Account<'info, TreeSpec>,
    #[account(mut, has_one=atelier)]
    pub journal: Account<'info, Journal>,
    pub mist: Signer<'info>,
}

#[account] pub struct Atelier { pub owner: Pubkey, pub water: u16, pub season: Season }
#[account] pub struct TreeSpec { pub atelier: Pubkey, pub twig: u8, pub vigor: u32 }
#[account] pub struct Journal { pub atelier: Pubkey, pub twig: u8, pub notes: u64, pub checksum: u32 }

#[error_code] pub enum BonsaiErr { #[msg("duplicate mutable account")] Dup }
