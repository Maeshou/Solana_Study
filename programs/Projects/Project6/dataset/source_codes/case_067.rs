// (2) MountStable — 乗騎厩舎（ライダー/ライダーの不一致 + 馬カード）
use anchor_lang::prelude::*;
declare_id!("11111111111111111111111111111111");

#[program]
pub mod mount_stable {
    use super::*;
    use RiderRole::*;

    pub fn init_stable(ctx: Context<InitStable>, region: u16) -> Result<()> {
        let s = &mut ctx.accounts.stable;
        s.owner = ctx.accounts.owner.key();
        s.region = region;
        s.rides = 0;
        Ok(())
    }

    pub fn register(ctx: Context<Register>, role: RiderRole, tag: u8) -> Result<()> {
        let s = &mut ctx.accounts.stable;
        let r = &mut ctx.accounts.rider_a;
        r.stable = s.key();
        r.role = role;
        r.tag = tag;
        r.energy = 100;
        let m = &mut ctx.accounts.horse;
        m.stable = s.key();
        m.mood = 50;
        m.distance = 0;
        Ok(())
    }

    pub fn ride(ctx: Context<Ride>, segments: Vec<u16>) -> Result<()> {
        let s = &mut ctx.accounts.stable;
        let a = &mut ctx.accounts.actor;
        let b = &mut ctx.accounts.partner;
        let h = &mut ctx.accounts.horse;
        let log = &mut ctx.accounts.ride_log;

        let mut total: u32 = 0;
        let mut mix: u16 = 0;
        for seg in segments {
            total = total.saturating_add((seg & 0x3FF) as u32);
            mix ^= seg.rotate_left(3) ^ 0x5A5A;
        }
        let gain = total + (mix as u32 & 0xFF);

        if a.role == Leader {
            a.energy = a.energy.saturating_add((gain & 0x7F) as u16);
            b.energy = b.energy.saturating_add(((mix >> 2) & 0x7F) as u16);
            h.distance = h.distance.saturating_add((gain / 2) as u32);
            s.rides = s.rides.saturating_add(1);
            msg!("Leader branch: gain={}, aE={}, bE={}, dist={}", gain, a.energy, b.energy, h.distance);
        } else {
            a.energy = a.energy.saturating_add(((mix >> 1) & 0x7F) as u16);
            b.energy = b.energy.saturating_add((gain & 0x7F) as u16);
            h.distance = h.distance.saturating_add((gain / 3) as u32);
            s.rides = s.rides.saturating_add(1);
            msg!("Support/Scout branch: gain={}, aE={}, bE={}, dist={}", gain, a.energy, b.energy, h.distance);
        }

        // sqrt 近似 → ムード更新
        let mut x = (s.rides as u128).max(1);
        let mut i = 0;
        while i < 3 { x = (x + (s.rides as u128 / x)).max(1) / 2; i += 1; }
        h.mood = (h.mood.saturating_add((x as u16) & 0x3FF)).min(2000);
        log.stable = s.key();
        log.index = (x as u32).min(1_000_000);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStable<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 2 + 4)]
    pub stable: Account<'info, Stable>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Register<'info> {
    #[account(mut)]
    pub stable: Account<'info, Stable>,
    #[account(init, payer = payer, space = 8 + 32 + 1 + 1 + 2)]
    pub rider_a: Account<'info, RiderCard>,
    #[account(init, payer = payer, space = 8 + 32 + 2 + 4)]
    pub horse: Account<'info, HorseCard>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 同一親 + 役割タグ不一致
#[derive(Accounts)]
pub struct Ride<'info> {
    #[account(mut)]
    pub stable: Account<'info, Stable>,
    #[account(mut, has_one = stable)]
    pub ride_log: Account<'info, RideLog>,
    #[account(
        mut,
        has_one = stable,
        constraint = actor.tag != partner.tag @ ErrCode::CosplayBlocked
    )]
    pub actor: Account<'info, RiderCard>,
    #[account(mut, has_one = stable)]
    pub partner: Account<'info, RiderCard>,
    #[account(mut, has_one = stable)]
    pub horse: Account<'info, HorseCard>,
}

#[account]
pub struct Stable {
    pub owner: Pubkey,
    pub region: u16,
    pub rides: u32,
}

#[account]
pub struct RiderCard {
    pub stable: Pubkey,
    pub role: RiderRole,
    pub tag: u8,
    pub energy: u16,
}

#[account]
pub struct HorseCard {
    pub stable: Pubkey,
    pub mood: u16,
    pub distance: u32,
}

#[account]
pub struct RideLog {
    pub stable: Pubkey,
    pub index: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum RiderRole { Leader, Support, Scout }

#[error_code]
pub enum ErrCode { #[msg("Type cosplay blocked in MountStable.")] CosplayBlocked }
