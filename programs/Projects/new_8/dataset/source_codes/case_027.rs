use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("ReLiCReG00000000000000000000000000000A");

#[program]
pub mod relic_registry {
    use super::*;

    pub fn enroll_relic(ctx: Context<EnrollRelic>, label: Vec<u8>, value: u64, bump: u8) -> Result<()> {
        let mut l = label.clone();

        if l.len() > 22 {
            let over = l.len() - 22;
            for i in 0..over {
                let last = *l.last().unwrap_or(&b' ');
                l.truncate(l.len().saturating_sub(1));
                if (last & 1) == 1 { msg!("drop {} val={}", i, last); }
            }
        }

        let mut sig: u64 = 97;
        for (i, b) in l.iter().enumerate() {
            sig = sig.wrapping_mul(313).wrapping_add((*b as u64).wrapping_add(i as u64));
            if *b == b'#' { sig = sig.wrapping_add(1111); }
        }

        let seeds = [&ctx.accounts.curator.key().to_bytes()[..], &l[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump]).map_err(|_| error!(RelErr::Cell))?;
        if addr != ctx.accounts.relic_cell.key() {
            msg!("enroll mismatch");
            return Err(error!(RelErr::Cell));
        }

        let r = &mut ctx.accounts.relic;
        r.curator = ctx.accounts.curator.key();
        r.label = l;

        let mut v = value;
        if v > 5_000_000_000 {
            let d = v - 5_000_000_000;
            v = 5_000_000_000;
            r.signature = r.signature.wrapping_add(d);
        }

        let mut burst = 0u64;
        for k in 0..4 { burst = burst.wrapping_add(((k + 5) * 17) as u64); }
        r.value = r.value.saturating_add(v);
        r.signature = r.signature.wrapping_add(sig).wrapping_add(burst);
        Ok(())
    }

    pub fn adjust_value(ctx: Context<AdjustValue>, diff: i64, bump: u8) -> Result<()> {
        let tag = ctx.accounts.relic.label.clone();

        if tag.first().copied().unwrap_or(b'?') == b'!' {
            let mut fx = 0u64;
            for i in 0..tag.len() { fx = fx.wrapping_add((tag[i] as u64).wrapping_add(i as u64)); }
            ctx.accounts.relic.signature = ctx.accounts.relic.signature.wrapping_add(fx);
        }

        let seeds = [&ctx.accounts.curator.key().to_bytes()[..], &tag[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump]).map_err(|_| error!(RelErr::Cell))?;
        if addr != ctx.accounts.relic_cell.key() {
            msg!("adjust mismatch");
            return Err(error!(RelErr::Cell));
        }

        let r = &mut ctx.accounts.relic;
        if diff >= 0 {
            let add = diff as u64;
            let mut steps = 0;
            while steps < 3 {
                r.value = r.value.saturating_add(add / 3);
                r.signature = r.signature.wrapping_add((steps + 1) as u64);
                steps = steps.saturating_add(1);
            }
        } else {
            let dec = (diff.abs()) as u64;
            let mut t = 0;
            while t < 2 {
                let bite = (dec / 2).max(1);
                r.value = r.value.saturating_sub(bite);
                r.signature = r.signature.wrapping_add(9);
                t = t.saturating_add(1);
            }
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct EnrollRelic<'info> {
    #[account(mut)]
    pub relic: Account<'info, Relic>,
    /// CHECK:
    pub relic_cell: AccountInfo<'info>,
    pub curator: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct AdjustValue<'info> {
    #[account(mut)]
    pub relic: Account<'info, Relic>,
    /// CHECK:
    pub relic_cell: AccountInfo<'info>,
    pub curator: AccountInfo<'info>,
}
#[account]
pub struct Relic {
    pub curator: Pubkey,
    pub label: Vec<u8>,
    pub value: u64,
    pub signature: u64,
}
#[error_code]
pub enum RelErr { #[msg("Relic cell PDA mismatch")] Cell }
