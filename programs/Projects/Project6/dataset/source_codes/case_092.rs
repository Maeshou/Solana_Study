// (7) Arena Ladder — ラダー順位と役割別カード
use anchor_lang::prelude::*;
declare_id!("Ar3n4Ladder777777777777777777777777777777");

#[program]
pub mod arena_ladder {
    use super::*;
    use Role::*;

    pub fn init_ladder(ctx: Context<InitLadder>) -> Result<()> {
        let l = &mut ctx.accounts.ladder;
        l.owner = ctx.accounts.host.key();
        l.epoch = 0;
        l.seed = 0;
        Ok(())
    }

    pub fn init_role_card(ctx: Context<InitRoleCard>, role: Role) -> Result<()> {
        let c = &mut ctx.accounts.card;
        c.ladder = ctx.accounts.ladder.key();
        c.role = role;
        c.points = 0;
        c.hist = [0; 8];
        Ok(())
    }

    pub fn run_epoch(ctx: Context<RunEpoch>, salt: u64) -> Result<()> {
        let l = &mut ctx.accounts.ladder;
        let a = &mut ctx.accounts.card_a;
        let b = &mut ctx.accounts.card_b;
        let t = &mut ctx.accounts.tally;

        let mut s = l.seed ^ salt;
        for i in 0..8 {
            s = s.rotate_left(9) ^ 0xD6E8FEB86659FD93u64;
            a.hist[i] = a.hist[i].saturating_add(((s >> (i * 7)) & 0x7F) as u32);
        }

        if a.role == Attacker {
            a.points = a.points.saturating_add(((s & 0xFFFF) as u32) + 5);
            l.epoch = l.epoch.saturating_add(1);
            t.lines = t.lines.saturating_add(1);
            t.mix = t.mix.wrapping_add(s.rotate_left(13));
            msg!("Attacker path applied");
        } else {
            b.points = b.points.saturating_add((((s >> 3) & 0x7FFF) as u32) + 3);
            l.epoch = l.epoch.saturating_add(1);
            t.lines = t.lines.saturating_add(2);
            t.mix = t.mix ^ s.rotate_right(11);
            msg!("Non-attacker path applied");
        }
        l.seed = s;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitLadder<'info> {
    #[account(init, payer = host, space = 8 + Ladder::MAX)]
    pub ladder: Account<'info, Ladder>,
    #[account(mut)]
    pub host: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct InitRoleCard<'info> {
    #[account(mut, has_one = owner, owner = crate::ID)]
    pub ladder: Account<'info, Ladder>,
    #[account(init, payer = user, space = 8 + RoleCard::MAX)]
    pub card: Account<'info, RoleCard>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct RunEpoch<'info> {
    #[account(mut, has_one = owner, owner = crate::ID)]
    pub ladder: Account<'info, Ladder>,
    #[account(mut, has_one = ladder, owner = crate::ID)]
    pub tally: Account<'info, Tally>,
    #[account(mut, has_one = ladder, owner = crate::ID)]
    pub card_a: Account<'info, RoleCard>,
    #[account(
        mut,
        has_one = ladder,
        owner = crate::ID,
        constraint = card_a.role != card_b.role @ ErrCode::CosplayBlocked
    )]
    pub card_b: Account<'info, RoleCard>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Ladder { pub owner: Pubkey, pub epoch: u64, pub seed: u64 }
impl Ladder { pub const MAX: usize = 32 + 8 + 8; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Role { Attacker, Defender, Support }
use Role::*;

#[account]
pub struct RoleCard { pub ladder: Pubkey, pub role: Role, pub points: u32, pub hist: [u32; 8] }
impl RoleCard { pub const MAX: usize = 32 + 1 + 4 + 4 * 8; }

#[account]
pub struct Tally { pub ladder: Pubkey, pub mix: u64, pub lines: u32 }
impl Tally { pub const MAX: usize = 32 + 8 + 4; }

#[error_code]
pub enum ErrCode { #[msg("Type Cosplay blocked by role mismatch")] CosplayBlocked }
