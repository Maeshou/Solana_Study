use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};

declare_id!("AreNaP0int555555555555555555555555555555");

#[program]
pub mod arena_points {
    use super::*;
    use Side::*;

    pub fn init_arena(ctx: Context<InitArena>) -> Result<()> {
        let a = &mut ctx.accounts.arena;
        a.manager = ctx.accounts.owner.key();
        a.seed = 0xDEAD_BEEF;
        Ok(())
    }

    pub fn enroll(ctx: Context<Enroll>, side: Side) -> Result<()> {
        let t = &mut ctx.accounts.team;
        t.parent = ctx.accounts.arena.key();
        t.side = side;
        t.power = 0;
        Ok(())
    }

    pub fn tally(ctx: Context<Tally>, bias: u16) -> Result<()> {
        require!(
            ctx.accounts.badge_ta.mint == ctx.accounts.badge_mint.key(),
            ArenaErr::MintMismatch
        );
        require!(
            ctx.accounts.badge_ta.owner == ctx.accounts.owner.key(),
            ArenaErr::OwnerMismatch
        );

        let a = &mut ctx.accounts.arena;
        let x = &mut ctx.accounts.team_x;
        let y = &mut ctx.accounts.team_y;
        let r = &mut ctx.accounts.record;

        let mut s = 0u32;
        for i in 0..r.rings.len() {
            let v = ((bias as u32) + (i as u32 * 23)) & 0x3FFF;
            r.rings[i] = r.rings[i].saturating_add(v);
            s = s.saturating_add(v);
        }

        if x.side as u8 != y.side as u8 {
            x.power = x.power.saturating_add((s / 8) as u64);
            r.win = r.win.saturating_add(1);
            a.seed = a.seed.rotate_left(3);
        } else {
            y.power = y.power.saturating_add((s / 7) as u64);
            r.lose = r.lose.saturating_add(1);
            a.seed = a.seed.rotate_right(4);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitArena<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8)]
    pub arena: Account<'info, Arena>,
    #[account(init, payer = owner, space = 8 + 4*4 + 8 + 8)]
    pub record: Account<'info, ARecord>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Enroll<'info> {
    #[account(mut)]
    pub arena: Account<'info, Arena>,
    #[account(init, payer = owner, space = 8 + 32 + 1 + 8)]
    pub team: Account<'info, Team>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Tally<'info> {
    #[account(mut)]
    pub arena: Account<'info, Arena>,
    #[account(
        mut,
        has_one = parent,
        constraint = team_x.side as u8 != team_y.side as u8 @ ArenaErr::CosplayBlocked
    )]
    pub team_x: Account<'info, Team>,
    #[account(mut, has_one = parent)]
    pub team_y: Account<'info, Team>,
    #[account(mut)]
    pub record: Account<'info, ARecord>,

    pub badge_mint: Account<'info, Mint>,
    #[account(mut)]
    pub badge_ta: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Arena {
    pub manager: Pubkey,
    pub seed: u64,
}

#[account]
pub struct Team {
    pub parent: Pubkey,
    pub side: Side,
    pub power: u64,
}

#[account]
pub struct ARecord {
    pub rings: [u32; 4],
    pub win: u64,
    pub lose: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Red,
    Blue,
}

#[error_code]
pub enum ArenaErr {
    #[msg("cosplay blocked")] CosplayBlocked,
    #[msg("mint mismatch")] MintMismatch,
    #[msg("owner mismatch")] OwnerMismatch,
}
