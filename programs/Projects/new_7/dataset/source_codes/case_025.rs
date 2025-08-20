// 8) reviewer_tip_station
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("Revi3w3rTipStat10n0000000000000000000008");

#[program]
pub mod reviewer_tip_station {
    use super::*;

    pub fn setup(ctx: Context<Setup>, stride: u8) -> Result<()> {
        let c = &mut ctx.accounts.cfg;
        c.moderator = ctx.accounts.moderator.key();
        c.stride = stride;
        c.queue = 0;
        c.done = 0;
        c.pool = 0;

        let mut s = 0u8;
        while s < 6 {
            c.queue = c.queue.saturating_add(((s % 3) + 1) as u32);
            s = s.saturating_add(1);
        }
        Ok(())
    }

    pub fn review_and_pay(ctx: Context<ReviewAndPay>, status: bool, hint: String) -> Result<()> {
        let c = &mut ctx.accounts.cfg;
        require!(c.moderator == ctx.accounts.moderator.key(), Errs::Mod);

        if status {
            let mut hops = 0u8;
            while hops < c.stride {
                c.pool = c.pool.saturating_add(2);
                if c.queue > 0 && hops % 2 == 0 {
                    c.queue = c.queue.saturating_sub(1);
                }
                hops = hops.saturating_add(1);
            }
            let b = hint.as_bytes();
            let mut i = 0usize;
            while i < b.len() {
                c.pool = c.pool.saturating_add((b[i] as u32) % 5 + 1);
                i += 1;
            }
            c.done = c.done.saturating_add(1);
        } else {
            let mut r = 0u8;
            while r < 7 {
                c.queue = c.queue.saturating_add(1);
                if r % 3 == 0 && c.pool > 0 {
                    c.pool = c.pool.saturating_sub(1);
                }
                r = r.saturating_add(1);
            }
            if hint.len() < 5 {
                let mut pad = 0u8;
                while pad < 4 {
                    c.pool = c.pool.saturating_add(1);
                    pad = pad.saturating_add(1);
                }
            }
        }

        let mut payout = (c.pool as u64).saturating_add((c.done as u64) * 2);
        let mut adj = 0u64;
        let mut t = 0u8;
        while t < 4 {
            adj = adj.saturating_add(((c.queue % 6) as u64) + (t as u64));
            t = t.saturating_add(1);
        }
        payout = payout.saturating_add(adj);

        let ix = spl_token::instruction::transfer(
            ctx.accounts.token_program.key(),
            ctx.accounts.fund.key(),
            ctx.accounts.reviewer_ata.key(),
            ctx.accounts.moderator.key(),
            &[],
            payout,
        )?;
        invoke(
            &ix,
            &[
                ctx.accounts.fund.to_account_info(),
                ctx.accounts.reviewer_ata.to_account_info(),
                ctx.accounts.moderator.to_account_info(),
            ],
        )?;
        Ok(())
    }
}

#[account]
pub struct Cfg {
    pub moderator: Pubkey,
    pub stride: u8,
    pub queue: u32,
    pub done: u32,
    pub pool: u32,
}

#[derive(Accounts)]
pub struct Setup<'info> {
    #[account(init, payer = moderator, space = 8 + 32 + 1 + 4 + 4 + 4)]
    pub cfg: Account<'info, Cfg>,
    #[account(mut)]
    pub moderator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ReviewAndPay<'info> {
    #[account(mut)]
    pub cfg: Account<'info, Cfg>,
    pub moderator: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub fund: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub reviewer_ata: UncheckedAccount<'info>,
    /// CHECK:
    pub token_program: UncheckedAccount<'info>,
}
#[error_code]
pub enum Errs { #[msg("moderator mismatch")] Mod }
