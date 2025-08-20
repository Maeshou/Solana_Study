// ============================================================================
// 1) Sigil Forge — グリフ融合（PDAあり; seeds固定 + has_one + constraint + assert_ne!）
// ============================================================================
declare_id!("SIGL111111111111111111111111111111111");
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum BenchState { Idle, Blend, Lockdown }

#[program]
pub mod sigil_forge {
    use super::*;

    pub fn init_forge(ctx: Context<InitForge>, cap: u32) -> Result<()> {
        let a = &mut ctx.accounts;
        a.workshop.artisan = a.artisan.key();
        a.workshop.capacity = cap;
        a.workshop.state = BenchState::Blend;
        // glyphやvaultの数値系はゼロ初期化に任せる
        Ok(())
    }

    pub fn fuse(ctx: Context<Fuse>, passes: u16) -> Result<()> {
        let a = &mut ctx.accounts;
        assert_ne!(a.vault.key(), a.glyph_a.key(), "vault/glyph_a must differ");
        assert_ne!(a.vault.key(), a.glyph_b.key(), "vault/glyph_b must differ");

        // ループ
        for i in 0..passes {
            let inc = 3 + (i % 5) as u32;
            a.glyph_a.ink = a.glyph_a.ink.saturating_add(inc);
            a.glyph_b.ink = a.glyph_b.ink.saturating_add(inc + 2);
            a.vault.sigils = a.vault.sigils.saturating_add(1);
        }

        // 分岐（4行以上）
        let total = a.glyph_a.ink.saturating_add(a.glyph_b.ink);
        if total > a.workshop.capacity {
            a.workshop.state = BenchState::Lockdown;
            a.vault.rarity = a.vault.rarity.saturating_add(7);
            a.vault.events = a.vault.events.saturating_add(2);
            msg!("capacity hit; lockdown, rarity+7, events+2");
        } else {
            a.workshop.state = BenchState::Blend;
            a.vault.rarity = a.vault.rarity.saturating_add(2);
            a.vault.sigils = a.vault.sigils.saturating_add(1);
            msg!("within capacity; blend, rarity+2, sigils+1");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitForge<'info> {
    #[account(init, payer=payer, space=8+32+4+1)]
    pub workshop: Account<'info, Workshop>,
    #[account(init, payer=payer, space=8+4+4)]
    pub glyph_a: Account<'info, Glyph>,
    #[account(init, payer=payer, space=8+4+4)]
    pub glyph_b: Account<'info, Glyph>,
    #[account(init, payer=payer, space=8+8+4, seeds=[b"vault", artisan.key().as_ref()], bump)]
    pub vault: Account<'info, Vault>,
    #[account(mut)] pub payer: Signer<'info>,
    pub artisan: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Fuse<'info> {
    #[account(mut, has_one=artisan)]
    pub workshop: Account<'info, Workshop>,
    #[account(mut, constraint = glyph_a.key() != glyph_b.key(), error = SFErr::Dup)]
    pub glyph_a: Account<'info, Glyph>,
    #[account(mut)]
    pub glyph_b: Account<'info, Glyph>,
    #[account(mut, seeds=[b"vault", artisan.key().as_ref()], bump)]
    pub vault: Account<'info, Vault>,
    pub artisan: Signer<'info>,
}

#[account] pub struct Workshop { pub artisan: Pubkey, pub capacity: u32, pub state: BenchState }
#[account] pub struct Glyph { pub ink: u32, pub resonance: u32 }
#[account] pub struct Vault { pub sigils: u64, pub rarity: u32, pub events: u32 }

#[error_code] pub enum SFErr { #[msg("duplicate mutable account")] Dup }
