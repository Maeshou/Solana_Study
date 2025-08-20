// (7) RhythmStudio — リズム工房（ビート職人/職人の不一致 + トラック盤）
use anchor_lang::prelude::*;
declare_id!("11111111111111111111111111111111");

#[program]
pub mod rhythm_studio {
    use super::*;
    use BeatRole::*;

    pub fn init_room(ctx: Context<InitRoom>, code: u32) -> Result<()> {
        let r = &mut ctx.accounts.room;
        r.owner = ctx.accounts.owner.key();
        r.code = code;
        r.tracks = 0;
        Ok(())
    }

    pub fn enroll(ctx: Context<Enroll>, role: BeatRole, slot: u8) -> Result<()> {
        let r = &mut ctx.accounts.room;
        let b = &mut ctx.accounts.beat;
        b.room = r.key();
        b.role = role;
        b.slot = slot;
        b.power = 0;
        let t = &mut ctx.accounts.track;
        t.room = r.key();
        t.energy = 0;
        Ok(())
    }

    pub fn compose(ctx: Context<Compose>, notes: Vec<u8>) -> Result<()> {
        let r = &mut ctx.accounts.room;
        let a = &mut ctx.accounts.actor;
        let p = &mut ctx.accounts.partner;
        let t = &mut ctx.accounts.track;
        let l = &mut ctx.accounts.timeline;

        let mut s: u32 = 0;
        let mut mix: u16 = 0;
        for n in notes {
            s = s.saturating_add((n & 0x3F) as u32);
            mix ^= (n as u16).rotate_left(2);
        }
        let base = s + (mix as u32 & 0xFF);

        if a.role == Drummer {
            a.power = a.power.saturating_add((base / 2) as u16);
            p.power = p.power.saturating_add(((mix >> 3) & 0x3F) as u16);
            t.energy = t.energy.saturating_add((base / 4) as u32);
            r.tracks = r.tracks.saturating_add(1);
            msg!("Drummer branch: base={}, aP={}, pP={}, e={}", base, a.power, p.power, t.energy);
        } else {
            a.power = a.power.saturating_add((base / 3) as u16);
            p.power = p.power.saturating_add(((mix >> 1) & 0x7F) as u16);
            t.energy = t.energy.saturating_add((base / 5) as u32);
            r.tracks = r.tracks.saturating_add(1);
            msg!("Bassist/Pianist branch: base={}, aP={}, pP={}, e={}", base, a.power, p.power, t.energy);
        }

        let mut x = (r.tracks as u128).max(1);
        let mut i = 0;
        while i < 3 { x = (x + (r.tracks as u128 / x)).max(1) / 2; i += 1; }
        l.room = r.key();
        l.cursor = (x as u32).min(1_000_000);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRoom<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 4)]
    pub room: Account<'info, Room>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Enroll<'info> {
    #[account(mut)]
    pub room: Account<'info, Room>,
    #[account(init, payer = payer, space = 8 + 32 + 1 + 1 + 2)]
    pub beat: Account<'info, BeatCard>,
    #[account(init, payer = payer, space = 8 + 32 + 4)]
    pub track: Account<'info, TrackBoard>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
// 同一親 + 役割タグ不一致
#[derive(Accounts)]
pub struct Compose<'info> {
    #[account(mut)]
    pub room: Account<'info, Room>,
    #[account(mut, has_one = room)]
    pub timeline: Account<'info, Timeline>,
    #[account(
        mut,
        has_one = room,
        constraint = actor.slot != partner.slot @ ErrCode::CosplayBlocked
    )]
    pub actor: Account<'info, BeatCard>,
    #[account(mut, has_one = room)]
    pub partner: Account<'info, BeatCard>,
    #[account(mut, has_one = room)]
    pub track: Account<'info, TrackBoard>,
}

#[account]
pub struct Room { pub owner: Pubkey, pub code: u32, pub tracks: u32 }

#[account]
pub struct BeatCard { pub room: Pubkey, pub role: BeatRole, pub slot: u8, pub power: u16 }

#[account]
pub struct TrackBoard { pub room: Pubkey, pub energy: u32 }

#[account]
pub struct Timeline { pub room: Pubkey, pub cursor: u32 }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum BeatRole { Drummer, Bassist, Pianist }

#[error_code]
pub enum ErrCode { #[msg("Type cosplay blocked in RhythmStudio.")] CosplayBlocked }
