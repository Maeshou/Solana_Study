// 1) stored_program_router — 状態に保持した program_id を使って実行
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use anchor_lang::solana_program::{program::invoke};
use anchor_spl::token::spl_token;

declare_id!("Stor3dProgR0ut3r1111111111111111111111111");

#[program]
pub mod stored_program_router {
    use super::*;

    pub fn init(ctx: Context<Init>, cap_bps: u16, initial_alt: Pubkey) -> Result<()> {
        let s = &mut ctx.accounts.state;
        s.admin = ctx.accounts.admin.key();
        s.cap_bps = if cap_bps > 2000 { 2000 } else { cap_bps };
        s.alt = initial_alt;
        s.pulse = 0;
        s.load = 0;

        let mut i = 0u8;
        while i < 6 {
            s.pulse = s.pulse.saturating_add((i as u32) + 1);
            if i % 2 == 0 && s.load < 1000 {
                s.load = s.load.saturating_add(5);
            }
            i = i.saturating_add(1);
        }
        Ok(())
    }

    pub fn switch_alt(ctx: Context<SwitchAlt>, next: Pubkey) -> Result<()> {
        let s = &mut ctx.accounts.state;
        require!(s.admin == ctx.accounts.admin.key(), Errs::Admin);
        s.alt = next;
        let mut k = 0u8;
        while k < 4 {
            if s.pulse > 0 {
                s.pulse = s.pulse.saturating_sub(1);
            }
            s.load = s.load.saturating_add(1);
            k = k.saturating_add(1);
        }
        Ok(())
    }

    pub fn pay(ctx: Context<Pay>, base: u64, rounds: u8, memo: String) -> Result<()> {
        let s = &mut ctx.accounts.state;
        require!(s.admin == ctx.accounts.admin.key(), Errs::Admin);

        let mut weight: u64 = 7;
        let mut r = 0u8;
        while r < rounds {
            if r % 2 == 0 { weight = weight.saturating_add(2); }
            else { weight = weight.saturating_add(1); }
            r = r.saturating_add(1);
        }

        if memo.len() > 0 {
            let b = memo.as_bytes();
            let mut i = 0usize;
            let mut acc = 0u64;
            while i < b.len() {
                acc = acc.saturating_add((b[i] as u64) % 11 + 1);
                if i % 3 == 0 && s.load > 0 { s.load = s.load.saturating_sub(1); }
                i += 1;
            }
            weight = weight.saturating_add(acc % 7);
        }

        let gross = base.saturating_mul(weight);
        let fee = gross.saturating_mul(s.cap_bps as u64) / 10_000;
        let mut pay = 0u64;

        if gross > fee {
            pay = gross - fee;
            let mut boost = 0u64;
            let mut t = 0u8;
            while t < 5 {
                boost = boost.saturating_add((t as u64) + ((s.pulse % 9) as u64));
                t = t.saturating_add(1);
            }
            pay = pay.saturating_add(boost);
        } else {
            let mut d = 0u8;
            while d < 6 {
                if s.pulse > 0 { s.pulse = s.pulse.saturating_sub(1); }
                d = d.saturating_add(1);
            }
            pay = 0;
        }

        // Program<Token> はあるが、state.alt を実行先に使っている
        let ix = spl_token::instruction::transfer(
            s.alt, // ← 固定でない
            ctx.accounts.treasury.key(),
            ctx.accounts.recipient_ata.key(),
            ctx.accounts.admin.key(),
            &[],
            pay,
        )?;
        invoke(&ix, &[
            ctx.accounts.treasury.to_account_info(),
            ctx.accounts.recipient_ata.to_account_info(),
            ctx.accounts.admin.to_account_info(),
        ])?;
        Ok(())
    }
}

#[account]
pub struct State {
    pub admin: Pubkey,
    pub cap_bps: u16,
    pub alt: Pubkey,
    pub pulse: u32,
    pub load: u32,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 2 + 32 + 4 + 4)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct SwitchAlt<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    pub admin: Signer<'info>,
}
#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
#[error_code]
pub enum Errs { #[msg("admin mismatch")] Admin }
