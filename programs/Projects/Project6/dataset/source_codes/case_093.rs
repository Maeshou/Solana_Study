// (8) Craft Workshop — 素材工房と配合レシピ
use anchor_lang::prelude::*;
declare_id!("CrAftW0rk5hop8888888888888888888888888888");

#[program]
pub mod craft_workshop {
    use super::*;
    use RecipeKind::*;

    pub fn init_workshop(ctx: Context<InitWorkshop>) -> Result<()> {
        let w = &mut ctx.accounts.workshop;
        w.owner = ctx.accounts.master.key();
        w.level = 1;
        w.noise = 0;
        Ok(())
    }

    pub fn init_recipe(ctx: Context<InitRecipe>, kind: RecipeKind) -> Result<()> {
        let r = &mut ctx.accounts.recipe;
        r.workshop = ctx.accounts.workshop.key();
        r.kind = kind;
        r.quality = 0;
        r.mem = [0; 10];
        Ok(())
    }

    pub fn brew(ctx: Context<Brew>, seed: u64) -> Result<()> {
        let w = &mut ctx.accounts.workshop;
        let a = &mut ctx.accounts.actor;
        let b = &mut ctx.accounts.counter;
        let l = &mut ctx.accounts.ledger;

        let mut s = seed ^ (w.level as u64);
        for k in 0..10 {
            s = (s ^ (k as u64).rotate_left(5)).wrapping_mul(0x9E3779B97F4A7C15);
            a.mem[k] = a.mem[k].saturating_add(((s >> (k * 3)) & 0x3F) as u32);
        }

        if a.kind == Elixir {
            a.quality = a.quality.saturating_add(((s & 0xFFFF) as u32) + 11);
            w.level = (w.level + 1).min(50);
            l.lines = l.lines.saturating_add(1);
            l.mix = l.mix.wrapping_add(s.rotate_left(7));
            msg!("Elixir brewing path");
        } else {
            b.quality = b.quality.saturating_add((((s >> 6) & 0x7FFF) as u32) + 5);
            w.noise = w.noise ^ ((s as u32) & 0xFFFF);
            l.lines = l.lines.saturating_add(2);
            l.mix = l.mix ^ s.rotate_right(9);
            msg!("Non-elixir brewing path");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitWorkshop<'info> {
    #[account(init, payer = master, space = 8 + Workshop::MAX)]
    pub workshop: Account<'info, Workshop>,
    #[account(mut)]
    pub master: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct InitRecipe<'info> {
    #[account(mut, has_one = owner, owner = crate::ID)]
    pub workshop: Account<'info, Workshop>,
    #[account(init, payer = user, space = 8 + Recipe::MAX)]
    pub recipe: Account<'info, Recipe>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Brew<'info> {
    #[account(mut, has_one = owner, owner = crate::ID)]
    pub workshop: Account<'info, Workshop>,
    #[account(mut, has_one = workshop, owner = crate::ID)]
    pub ledger: Account<'info, Ledger>,
    #[account(mut, has_one = workshop, owner = crate::ID)]
    pub actor: Account<'info, Recipe>,
    #[account(
        mut,
        has_one = workshop,
        owner = crate::ID,
        constraint = actor.kind != counter.kind @ ErrCode::CosplayBlocked
    )]
    pub counter: Account<'info, Recipe>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Workshop { pub owner: Pubkey, pub level: u16, pub noise: u32 }
impl Workshop { pub const MAX: usize = 32 + 2 + 4; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum RecipeKind { Elixir, Alloy, Ink }
use RecipeKind::*;

#[account]
pub struct Recipe { pub workshop: Pubkey, pub kind: RecipeKind, pub quality: u32, pub mem: [u32; 10] }
impl Recipe { pub const MAX: usize = 32 + 1 + 4 + 4 * 10; }

#[account]
pub struct Ledger { pub workshop: Pubkey, pub mix: u64, pub lines: u32 }
impl Ledger { pub const MAX: usize = 32 + 8 + 4; }

#[error_code]
pub enum ErrCode { #[msg("Type Cosplay blocked by recipe kind mismatch")] CosplayBlocked }
