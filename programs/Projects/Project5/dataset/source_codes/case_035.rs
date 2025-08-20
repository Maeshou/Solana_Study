
// ============================================================================
// 3) Crystal Orchard — クリスタル果樹園（固定小数点Q16.16）— PDAあり
// ============================================================================
declare_id!("CROR333333333333333333333333333333333");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum OrchardState { Seed, Bloom, Prune }

#[program]
pub mod crystal_orchard {
    use super::*;

    pub fn init_orchard(ctx: Context<InitOrchard>, cap: u32) -> Result<()> {
        let o = &mut ctx.accounts;
        o.cfg.caretaker = o.caretaker.key();
        o.cfg.cap = cap;
        o.cfg.state = OrchardState::Seed;
        Ok(())
    }

    pub fn irrigate(ctx: Context<Irrigate>, days: u32, rate_q16: u32) -> Result<()> {
        let o = &mut ctx.accounts;
        let rate = rate_q16.max(1); // Q16.16の分子（1以上）
        for _ in 0..days {
            // growth = growth + growth*rate (Q16.16)
            let g = (u128::from(o.tree.growth_q16) * u128::from(rate)) >> 16;
            o.tree.growth_q16 = (u128::from(o.tree.growth_q16) + g)
                .min(u128::from(u64::MAX))
                as u64;
            // color は段階的に12bitロール
            o.tree.color = o.tree.color.rotate_left(3) ^ (o.tree.color >> 5);
        }

        let growth = (o.tree.growth_q16 >> 16) as u32;
        if growth > o.cfg.cap {
            o.cfg.state = OrchardState::Prune;
            o.journal.cuts = o.journal.cuts.wrapping_add(2);
            o.tree.growth_q16 = u64::from(o.cfg.cap) << 16; // クリップ
            msg!("prune: clip growth to cap, cuts+2");
        } else {
            o.cfg.state = OrchardState::Bloom;
            o.journal.notes = o.journal.notes.wrapping_add(1);
            o.tree.color ^= 0x0F0F;
            msg!("bloom: notes+1, color xor");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitOrchard<'info> {
    #[account(init, payer=payer, space=8+32+4+1, seeds=[b"cfg", caretaker.key().as_ref()], bump)]
    pub cfg: Account<'info, OrchardCfg>,
    #[account(init, payer=payer, space=8+8+4)]
    pub tree: Account<'info, CrystalTree>,
    #[account(init, payer=payer, space=8+4+4)]
    pub journal: Account<'info, OrchardJournal>,
    #[account(mut)] pub payer: Signer<'info>,
    pub caretaker: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Irrigate<'info> {
    #[account(mut, seeds=[b"cfg", caretaker.key().as_ref()], bump, has_one=caretaker)]
    pub cfg: Account<'info, OrchardCfg>,
    #[account(mut, constraint = cfg.key() != tree.key(), error = OrchardErr::Dup)]
    pub tree: Account<'info, CrystalTree>,
    #[account(mut)]
    pub journal: Account<'info, OrchardJournal>,
    pub caretaker: Signer<'info>,
}

#[account] pub struct OrchardCfg { pub caretaker: Pubkey, pub cap: u32, pub state: OrchardState }
#[account] pub struct CrystalTree { pub growth_q16: u64, pub color: u32 }
#[account] pub struct OrchardJournal { pub cuts: u32, pub notes: u32 }

#[error_code] pub enum OrchardErr { #[msg("dup")] Dup }

