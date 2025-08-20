// (3) PetGarden — ペット庭園（トレーナー/トレーナーのロール不一致 + ペット）
use anchor_lang::prelude::*;
declare_id!("11111111111111111111111111111111");

#[program]
pub mod pet_garden {
    use super::*;
    use TrainerRole::*;

    pub fn init_garden(ctx: Context<InitGarden>, zone: u8) -> Result<()> {
        let g = &mut ctx.accounts.garden;
        g.owner = ctx.accounts.owner.key();
        g.zone = zone;
        g.visits = 0;
        Ok(())
    }

    pub fn add_pairs(ctx: Context<AddPairs>, role: TrainerRole, pet_kind: u8) -> Result<()> {
        let g = &mut ctx.accounts.garden;
        let t = &mut ctx.accounts.trainer;
        t.garden = g.key();
        t.role = role;
        t.tag = pet_kind;
        t.apt = 0;
        let p = &mut ctx.accounts.pet;
        p.garden = g.key();
        p.kind = pet_kind;
        p.affection = 0;
        Ok(())
    }

    pub fn train(ctx: Context<Train>, reps: Vec<u8>) -> Result<()> {
        let g = &mut ctx.accounts.garden;
        let a = &mut ctx.accounts.actor;
        let b = &mut ctx.accounts.partner;
        let p = &mut ctx.accounts.pet;
        let l = &mut ctx.accounts.session_log;

        let mut s: u32 = 0;
        let mut mix: u16 = 0;
        for r in reps {
            s = s.saturating_add((r & 0x7F) as u32);
            mix ^= (r as u16).rotate_left(1);
        }
        let base = s + (mix & 0xFF) as u32;

        if a.role == Mentor {
            a.apt = a.apt.saturating_add((base / 2) as u16);
            b.apt = b.apt.saturating_add(((mix >> 2) & 0x3F) as u16);
            p.affection = p.affection.saturating_add((base / 4) as u16);
            g.visits = g.visits.saturating_add(1);
            msg!("Mentor: base={}, a.apt={}, b.apt={}, pet.aff={}", base, a.apt, b.apt, p.affection);
        } else {
            a.apt = a.apt.saturating_add((base / 3) as u16);
            b.apt = b.apt.saturating_add(((mix >> 3) & 0x3F) as u16);
            p.affection = p.affection.saturating_add((base / 5) as u16);
            g.visits = g.visits.saturating_add(1);
            msg!("Assistant/Guest: base={}, a.apt={}, b.apt={}, pet.aff={}", base, a.apt, b.apt, p.affection);
        }

        let mut x = (g.visits as u128).max(1);
        let mut i = 0;
        while i < 3 { x = (x + (g.visits as u128 / x)).max(1) / 2; i += 1; }
        l.garden = g.key();
        l.index = (x as u32).min(1_000_000);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitGarden<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 1 + 4)]
    pub garden: Account<'info, Garden>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddPairs<'info> {
    #[account(mut)]
    pub garden: Account<'info, Garden>,
    #[account(init, payer = payer, space = 8 + 32 + 1 + 1 + 2)]
    pub trainer: Account<'info, TrainerCard>,
    #[account(init, payer = payer, space = 8 + 32 + 1 + 2)]
    pub pet: Account<'info, PetCard>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 同一親 + 役割タグ不一致
#[derive(Accounts)]
pub struct Train<'info> {
    #[account(mut)]
    pub garden: Account<'info, Garden>,
    #[account(mut, has_one = garden)]
    pub session_log: Account<'info, SessionLog>,
    #[account(
        mut,
        has_one = garden,
        constraint = actor.tag != partner.tag @ ErrCode::CosplayBlocked
    )]
    pub actor: Account<'info, TrainerCard>,
    #[account(mut, has_one = garden)]
    pub partner: Account<'info, TrainerCard>,
    #[account(mut, has_one = garden)]
    pub pet: Account<'info, PetCard>,
}

#[account]
pub struct Garden { pub owner: Pubkey, pub zone: u8, pub visits: u32 }

#[account]
pub struct TrainerCard { pub garden: Pubkey, pub role: TrainerRole, pub tag: u8, pub apt: u16 }

#[account]
pub struct PetCard { pub garden: Pubkey, pub kind: u8, pub affection: u16 }

#[account]
pub struct SessionLog { pub garden: Pubkey, pub index: u32 }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum TrainerRole { Mentor, Assistant, Guest }

#[error_code]
pub enum ErrCode { #[msg("Type cosplay blocked in PetGarden.")] CosplayBlocked }
