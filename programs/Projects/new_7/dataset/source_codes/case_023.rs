// 6) marketplace_cashback_pool
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("Mark3tCashb4ckP00l00000000000000000000006");

#[program]
pub mod marketplace_cashback_pool {
    use super::*;

    pub fn open(ctx: Context<Open>, cap_bps: u16) -> Result<()> {
        let s = &mut ctx.accounts.state;
        s.owner = ctx.accounts.owner.key();
        s.cap_bps = if cap_bps > 2500 { 2500 } else { cap_bps };
        s.trades = 0;
        s.cashback = 0;
        s.salt = 0;

        let seed = s.owner.as_ref()[0] as u32;
        let mut i = 0u8;
        while i < 7 {
            s.salt = s.salt.saturating_add(((seed + i as u32) % 5) + 1);
            i = i.saturating_add(1);
        }
        Ok(())
    }

    pub fn fill_and_cashback(ctx: Context<FillAndCashback>, price: u64, qty: u64, tag: String) -> Result<()> {
        let s = &mut ctx.accounts.state;
        require!(s.owner == ctx.accounts.owner.key(), Errs::Owner);

        s.trades = s.trades.saturating_add(1);

        let value = price.saturating_mul(qty);
        let mut cb = value.saturating_mul(s.cap_bps as u64) / 10_000;

        if s.trades % 2 == 0 {
            let b = tag.as_bytes();
            let mut i = 0usize;
            let mut wave = 0u64;
            while i < b.len() {
                wave = wave.saturating_add((b[i] as u64) % 7 + 1);
                if i % 4 == 0 { wave = wave.saturating_add(2); }
                i += 1;
            }
            let mut j = 0u8;
            while j < 5 {
                cb = cb.saturating_add((j as u64) + (wave % 3));
                if cb > value { cb = value; }
                j = j.saturating_add(1);
            }
        } else {
            let mut d = 0u8;
            while d < 6 {
                if cb > (value / 25) { cb = cb.saturating_sub(((d + 1) as u64)); }
                d = d.saturating_add(1);
            }
            if tag.len() < 6 {
                let mut bump = 0u8;
                while bump < 4 {
                    cb = cb.saturating_add(1);
                    bump = bump.saturating_add(1);
                }
            }
        }

        s.cashback = s.cashback.saturating_add(cb);

        let ix = spl_token::instruction::transfer(
            ctx.accounts.token_program.key(),
            ctx.accounts.fee_vault.key(),
            ctx.accounts.trader_ata.key(),
            ctx.accounts.owner.key(),
            &[],
            cb,
        )?;
        invoke(
            &ix,
            &[
                ctx.accounts.fee_vault.to_account_info(),
                ctx.accounts.trader_ata.to_account_info(),
                ctx.accounts.owner.to_account_info(),
            ],
        )?;
        Ok(())
    }
}

#[account]
pub struct State {
    pub owner: Pubkey,
    pub cap_bps: u16,
    pub trades: u32,
    pub cashback: u64,
    pub salt: u32,
}

#[derive(Accounts)]
pub struct Open<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 2 + 4 + 8 + 4)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct FillAndCashback<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    pub owner: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub fee_vault: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub trader_ata: UncheckedAccount<'info>,
    /// CHECK:
    pub token_program: UncheckedAccount<'info>,
}
#[error_code]
pub enum Errs { #[msg("owner mismatch")] Owner }
