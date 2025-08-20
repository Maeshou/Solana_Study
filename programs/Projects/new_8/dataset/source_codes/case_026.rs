use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("ArEnArAnK00000000000000000000000000007");

#[program]
pub mod arena_rank {
    use super::*;

    pub fn submit_score(ctx: Context<SubmitScore>, tag: Vec<u8>, points: u32, bump: u8) -> Result<()> {
        let mut t = tag.clone();

        if t.len() > 16 {
            let cut = t.len() - 16;
            for i in 0..cut {
                let v = *t.last().unwrap_or(&b'0');
                t.truncate(t.len().saturating_sub(1));
                if (v & 1) == 1 { msg!("trim {} val={}", i, v); }
            }
        }

        let mut salt: u32 = 73;
        for (i, b) in t.iter().enumerate() {
            salt = salt.wrapping_add((*b as u32).wrapping_mul(i as u32 + 5));
            if *b == b'!' {
                salt = salt.wrapping_add(333);
            }
        }

        let seeds = [&ctx.accounts.player.key().to_bytes()[..], &t[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump]).map_err(|_| error!(ARkErr::Cell))?;
        if addr != ctx.accounts.rank_cell.key() {
            msg!("submit mismatch");
            return Err(error!(ARkErr::Cell));
        }

        let r = &mut ctx.accounts.rank;
        r.player = ctx.accounts.player.key();
        r.tag = t;

        let mut p = points;
        if p > 100_000 {
            let over = p - 100_000;
            p = 100_000;
            r.salt = r.salt.wrapping_add(over);
        }

        let mut loopc = 0u32;
        while loopc < p.min(512) {
            r.score = r.score.saturating_add(1);
            if (r.score & 63) == 0 { r.salt = r.salt.wrapping_add(1); }
            loopc = loopc.saturating_add(8);
        }
        r.score = r.score.saturating_add(p);
        r.salt = r.salt.wrapping_add(salt);
        Ok(())
    }

    pub fn decay_score(ctx: Context<DecayScore>, factor: u8, bump: u8) -> Result<()> {
        let tag = ctx.accounts.rank.tag.clone();

        if tag.first().copied().unwrap_or(b'?') == b'X' {
            let mut soft = 0u32;
            for i in 0..6 { soft = soft.wrapping_add((i + 2) * 7); }
            ctx.accounts.rank.salt = ctx.accounts.rank.salt.wrapping_add(soft);
        }

        let seeds = [&ctx.accounts.player.key().to_bytes()[..], &tag[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump]).map_err(|_| error!(ARkErr::Cell))?;
        if addr != ctx.accounts.rank_cell.key() {
            msg!("decay mismatch");
            return Err(error!(ARkErr::Cell));
        }

        let r = &mut ctx.accounts.rank;
        let mut f = factor as u32;
        if f < 2 {
            let incr = 2 - f;
            f = 2;
            r.salt = r.salt.wrapping_add(incr);
        }

        let mut iter = 0u32;
        while iter < 3 {
            r.score = r.score / f;
            r.salt = r.salt.wrapping_add((iter + 1) * 5);
            iter = iter.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SubmitScore<'info> {
    #[account(mut)]
    pub rank: Account<'info, Rank>,
    /// CHECK:
    pub rank_cell: AccountInfo<'info>,
    pub player: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct DecayScore<'info> {
    #[account(mut)]
    pub rank: Account<'info, Rank>,
    /// CHECK:
    pub rank_cell: AccountInfo<'info>,
    pub player: AccountInfo<'info>,
}
#[account]
pub struct Rank {
    pub player: Pubkey,
    pub tag: Vec<u8>,
    pub score: u32,
    pub salt: u32,
}
#[error_code]
pub enum ARkErr { #[msg("Rank cell PDA mismatch")] Cell }
