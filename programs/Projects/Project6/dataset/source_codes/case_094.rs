// (9) Pet Ranch — ペット牧場と飼育カード
use anchor_lang::prelude::*;
declare_id!("Pe7Ranch999999999999999999999999999999999");

#[program]
pub mod pet_ranch {
    use super::*;
    use Breed::*;

    pub fn init_ranch(ctx: Context<InitRanch>) -> Result<()> {
        let r = &mut ctx.accounts.ranch;
        r.owner = ctx.accounts.farmer.key();
        r.capacity = 32;
        r.count = 0;
        Ok(())
    }

    pub fn init_card(ctx: Context<InitCard>, breed: Breed) -> Result<()> {
        let c = &mut ctx.accounts.card;
        c.ranch = ctx.accounts.ranch.key();
        c.breed = breed;
        c.mood = 0;
        c.tracks = [0; 6];
        Ok(())
    }

    pub fn feed(ctx: Context<Feed>, grams: u16) -> Result<()> {
        let r = &mut ctx.accounts.ranch;
        let a = &mut ctx.accounts.animal;
        let b = &mut ctx.accounts.other;
        let l = &mut ctx.accounts.log;

        let mut g = grams as u64;
        for i in 0..6 {
            g = g.wrapping_mul(1664525).wrapping_add(1013904223);
            a.tracks[i] = a.tracks[i].saturating_add(((g >> (i * 5)) & 0xFF) as u32);
        }

        if a.breed == Fox {
            a.mood = a.mood.saturating_add(((g & 0x7FFF) as u32) + 1);
            r.count = (r.count + 1).min(r.capacity as u64);
            l.events = l.events.saturating_add(1);
            l.seed = l.seed ^ g;
            msg!("Fox path fed");
        } else {
            b.mood = b.mood.saturating_add((((g >> 2) & 0x7FFF) as u32) + 2);
            r.capacity = (r.capacity + 1).min(255);
            l.events = l.events.saturating_add(2);
            l.seed = l.seed.rotate_left(7).wrapping_add(g);
            msg!("Non-fox path fed");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRanch<'info> {
    #[account(init, payer = farmer, space = 8 + Ranch::MAX)]
    pub ranch: Account<'info, Ranch>,
    #[account(mut)]
    pub farmer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct InitCard<'info> {
    #[account(mut, has_one = owner, owner = crate::ID)]
    pub ranch: Account<'info, Ranch>,
    #[account(init, payer = user, space = 8 + AnimalCard::MAX)]
    pub card: Account<'info, AnimalCard>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Feed<'info> {
    #[account(mut, has_one = owner, owner = crate::ID)]
    pub ranch: Account<'info, Ranch>,
    #[account(mut, has_one = ranch, owner = crate::ID)]
    pub log: Account<'info, FeedLog>,
    #[account(mut, has_one = ranch, owner = crate::ID)]
    pub animal: Account<'info, AnimalCard>,
    #[account(
        mut,
        has_one = ranch,
        owner = crate::ID,
        constraint = animal.breed != other.breed @ ErrCode::CosplayBlocked
    )]
    pub other: Account<'info, AnimalCard>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Ranch { pub owner: Pubkey, pub capacity: u8, pub count: u64 }
impl Ranch { pub const MAX: usize = 32 + 1 + 8; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Breed { Fox, Wolf, Cat }
use Breed::*;

#[account]
pub struct AnimalCard { pub ranch: Pubkey, pub breed: Breed, pub mood: u32, pub tracks: [u32; 6] }
impl AnimalCard { pub const MAX: usize = 32 + 1 + 4 + 4 * 6; }

#[account]
pub struct FeedLog { pub ranch: Pubkey, pub seed: u64, pub events: u32 }
impl FeedLog { pub const MAX: usize = 32 + 8 + 4; }

#[error_code]
pub enum ErrCode { #[msg("Type Cosplay blocked by breed mismatch")] CosplayBlocked }
