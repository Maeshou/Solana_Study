// (4) RuneAlchemy — ルーン調合（術者/術者のロール不一致 + 釜と記録）
use anchor_lang::prelude::*;
declare_id!("11111111111111111111111111111111");

#[program]
pub mod rune_alchemy {
    use super::*;
    use Caster::*;

    pub fn init_cauldron(ctx: Context<InitCauldron>, seed: u64) -> Result<()> {
        let c = &mut ctx.accounts.cauldron;
        c.owner = ctx.accounts.owner.key();
        c.seed = seed;
        c.brews = 0;
        Ok(())
    }

    pub fn enroll_caster(ctx: Context<EnrollCaster>, kind: Caster, sigil: u16) -> Result<()> {
        let c = &mut ctx.accounts.cauldron;
        let a = &mut ctx.accounts.adept;
        a.cauldron = c.key();
        a.kind = kind;
        a.sigil = sigil;
        a.power = 0;
        Ok(())
    }

    pub fn brew(ctx: Context<Brew>, shards: Vec<u16>) -> Result<()> {
        let c = &mut ctx.accounts.cauldron;
        let x = &mut ctx.accounts.actor;
        let y = &mut ctx.accounts.partner;
        let v = &mut ctx.accounts.vessel;
        let l = &mut ctx.accounts.brew_log;

        let mut s: u32 = 0;
        let mut mix: u16 = 0;
        for sh in shards {
            let q = (sh & 0x3FF) as u32;
            s = s.saturating_add(q);
            mix ^= (sh.rotate_left(2)) ^ 0x0F0F;
        }
        let base = s + (mix as u32 & 0xFF);

        if x.kind == Arch {
            x.power = x.power.saturating_add((base / 2) as u16);
            y.power = y.power.saturating_add(((mix >> 3) & 0x3F) as u16);
            v.quality = v.quality.saturating_add((base / 4) as u16);
            c.brews = c.brews.saturating_add(1);
            msg!("Arch branch: base={}, xP={}, yP={}, q={}", base, x.power, y.power, v.quality);
        } else {
            x.power = x.power.saturating_add((base / 3) as u16);
            y.power = y.power.saturating_add(((mix >> 1) & 0x7F) as u16);
            v.quality = v.quality.saturating_add((base / 5) as u16);
            c.brews = c.brews.saturating_add(1);
            msg!("Mage/Novice branch: base={}, xP={}, yP={}, q={}", base, x.power, y.power, v.quality);
        }

        let mut t = (c.brews as u128).max(1);
        let mut i = 0;
        while i < 3 { t = (t + (c.brews as u128 / t)).max(1) / 2; i += 1; }
        l.cauldron = c.key();
        l.index = (t as u32).min(1_000_000);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitCauldron<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 4)]
    pub cauldron: Account<'info, Cauldron>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EnrollCaster<'info> {
    #[account(mut)]
    pub cauldron: Account<'info, Cauldron>,
    #[account(init, payer = payer, space = 8 + 32 + 1 + 2 + 2)]
    pub adept: Account<'info, AdeptCard>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 同一親 + 術者ロール不一致
#[derive(Accounts)]
pub struct Brew<'info> {
    #[account(mut)]
    pub cauldron: Account<'info, Cauldron>,
    #[account(mut, has_one = cauldron)]
    pub brew_log: Account<'info, BrewLog>,
    #[account(
        mut,
        has_one = cauldron,
        constraint = actor.kind != partner.kind @ ErrCode::CosplayBlocked
    )]
    pub actor: Account<'info, AdeptCard>,
    #[account(mut, has_one = cauldron)]
    pub partner: Account<'info, AdeptCard>,
    #[account(mut, has_one = cauldron)]
    pub vessel: Account<'info, Vessel>,
}

#[account]
pub struct Cauldron { pub owner: Pubkey, pub seed: u64, pub brews: u32 }

#[account]
pub struct AdeptCard { pub cauldron: Pubkey, pub kind: Caster, pub sigil: u16, pub power: u16 }

#[account]
pub struct Vessel { pub cauldron: Pubkey, pub quality: u16 }

#[account]
pub struct BrewLog { pub cauldron: Pubkey, pub index: u32 }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Caster { Novice, Mage, Arch }

#[error_code]
pub enum ErrCode { #[msg("Type cosplay blocked in RuneAlchemy.")] CosplayBlocked }
