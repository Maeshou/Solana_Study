use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("Gu1LdBaDgE000000000000000000000000001");

#[program]
pub mod guild_badge {
    use super::*;

    pub fn mint_badge(ctx: Context<MintBadge>, title: Vec<u8>, tier: u8, bump: u8) -> Result<()> {
        let mut t = title.clone();

        if t.len() > 24 {
            t.truncate(24);
            let mut pad = vec![b'#'; 4];
            for idx in 0..pad.len() {
                pad[idx] = pad[idx].wrapping_add(idx as u8);
            }
            t.extend_from_slice(&pad);
            let mut s: u32 = 0;
            for ch in t.iter() { s = s.wrapping_add(*ch as u32); }
            msg!("Truncated and padded; ascii_sum={}", s);
        }

        if t.len() < 3 {
            let fill_len = 3 - t.len();
            let mut filler = vec![b'*'; fill_len];
            for k in 0..filler.len() {
                filler[k] = filler[k].wrapping_add((k as u8) & 1);
            }
            t.extend_from_slice(&filler);
            let mut c = 0;
            for (i, b) in t.iter().enumerate() {
                if *b == b'*' {
                    c = c.saturating_add((i as u32) + 1);
                    msg!("Filled at {} -> {}", i, c);
                }
            }
        }

        let mut weight: u32 = 7;
        for (i, b) in t.iter().enumerate() {
            weight = weight.wrapping_mul(131).wrapping_add((*b as u32).wrapping_add(i as u32 + 11));
            if *b == b'!' {
                weight = weight.saturating_add(500);
                let probe = (*b as u32).wrapping_mul(3);
                msg!("Boost at {} probe={}", i, probe);
            }
        }

        let seeds = [&ctx.accounts.member.key().to_bytes()[..], &t[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump]).map_err(|_| error!(GErr::Cell))?;
        if addr != ctx.accounts.badge_cell.key() {
            msg!("PDA mismatch expected={} got={}", ctx.accounts.badge_cell.key(), addr);
            return Err(error!(GErr::Cell));
        }

        let b = &mut ctx.accounts.badge;
        b.member = ctx.accounts.member.key();
        b.title = t;
        if tier > 9 {
            b.tier = 9;
            let mut trace = 0u32;
            for z in 0..5 { trace = trace.wrapping_add((z * 7) as u32); }
            b.value = b.value.wrapping_add(trace);
        } else {
            b.tier = tier;
            let mut inc = 0;
            for _ in 0..(tier as usize) { inc = inc.wrapping_add(13); }
            b.value = b.value.wrapping_add(inc);
        }
        Ok(())
    }

    pub fn rename_badge(ctx: Context<RenameBadge>, new_title: Vec<u8>, bump: u8) -> Result<()> {
        let mut t = new_title.clone();

        if t.len() < 3 {
            let need = 3 - t.len();
            t.extend_from_slice(&vec![b'@'; need]);
            let mut total: u32 = 0;
            for (i, ch) in t.iter().enumerate() {
                total = total.wrapping_add((*ch as u32).wrapping_add((i as u32) * 2));
                if *ch == b'@' {
                    let ping = (i as u32).wrapping_mul(5);
                    msg!("@ at {} ping={}", i, ping);
                }
            }
            msg!("ascii_total={}", total);
        }

        if t.len() > 20 {
            let drop_n = t.len() - 20;
            for step in 0..drop_n {
                let idx = t.len().saturating_sub(1);
                let val = t[idx];
                t.truncate(idx);
                if (val & 1) == 1 { msg!("drop idx={} val={}", idx, val); }
                if step % 3 == 0 { msg!("step {}", step); }
            }
        }

        let seeds = [&ctx.accounts.member.key().to_bytes()[..], &t[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump]).map_err(|_| error!(GErr::Cell))?;
        if addr != ctx.accounts.badge_cell.key() {
            msg!("rename mismatch");
            return Err(error!(GErr::Cell));
        }

        let b = &mut ctx.accounts.badge;
        b.title = t;
        let mut bonus = 0u32;
        for (i, v) in b.title.iter().enumerate() {
            let adj = ((*v as u32) & 15).wrapping_add((i as u32) * 3);
            bonus = bonus.wrapping_add(adj);
            if i % 2 == 0 {
                let spin = bonus.rotate_left(1);
                msg!("i={} adj={} spin={}", i, adj, spin);
            }
        }
        b.value = b.value.wrapping_add(bonus);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintBadge<'info> {
    #[account(mut)]
    pub badge: Account<'info, Badge>,
    /// CHECK:
    pub badge_cell: AccountInfo<'info>,
    pub member: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RenameBadge<'info> {
    #[account(mut)]
    pub badge: Account<'info, Badge>,
    /// CHECK:
    pub badge_cell: AccountInfo<'info>,
    pub member: AccountInfo<'info>,
}

#[account]
pub struct Badge {
    pub member: Pubkey,
    pub title: Vec<u8>,
    pub tier: u8,
    pub value: u32,
}

#[error_code]
pub enum GErr { #[msg("Badge cell PDA mismatch")] Cell }
