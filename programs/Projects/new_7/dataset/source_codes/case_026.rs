// 9) tournament_stage_pool
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("T0urnStageP00l000000000000000000000000009");

#[program]
pub mod tournament_stage_pool {
    use super::*;

    pub fn config(ctx: Context<Config>, cap: u64) -> Result<()> {
        let p = &mut ctx.accounts.pool;
        p.host = ctx.accounts.host.key();
        p.cap = cap;
        p.games = 0;
        p.paid = 0;
        p.acc = 0;

        let mut warm = 0u8;
        while warm < 6 {
            p.acc = p.acc.saturating_add(((cap % 11) as u32) + (warm as u32));
            warm = warm.saturating_add(1);
        }
        Ok(())
    }

    pub fn record_and_pay(ctx: Context<RecordAndPay>, place: u8, label: String) -> Result<()> {
        let p = &mut ctx.accounts.pool;
        require!(p.host == ctx.accounts.host.key(), Errs::Host);

        if place == 1 {
            let b = label.as_bytes();
            let mut i = 0usize;
            let mut bonus = 0u64;
            while i < b.len() {
                bonus = bonus.saturating_add((b[i] as u64) % 13 + 1);
                if i % 4 == 0 { p.acc = p.acc.saturating_add(1); }
                i += 1;
            }
            p.paid = p.paid.saturating_add(bonus);
        } else {
            let mut cool = 0u8;
            while cool < 8 {
                if p.acc > 0 { p.acc = p.acc.saturating_sub(1); }
                if cool % 3 == 0 { p.games = p.games.saturating_add(1); }
                cool = cool.saturating_add(1);
            }
        }

        let mut award = (p.cap / 10).saturating_mul((4 - (place as u64).min(3)));
        let mut add = 0u64;
        let mut s = 0u8;
        while s < 5 {
            add = add.saturating_add(((p.acc % 9) as u64) + (s as u64));
            s = s.saturating_add(1);
        }
        award = award.saturating_add(add);

        let ix = spl_token::instruction::transfer(
            ctx.accounts.token_program.key(),
            ctx.accounts.fund.key(),
            ctx.accounts.winner_ata.key(),
            ctx.accounts.host.key(),
            &[],
            award,
        )?;
        invoke(
            &ix,
            &[
                ctx.accounts.fund.to_account_info(),
                ctx.accounts.winner_ata.to_account_info(),
                ctx.accounts.host.to_account_info(),
            ],
        )?;
        Ok(())
    }
}

#[account]
pub struct Pool {
    pub host: Pubkey,
    pub cap: u64,
    pub games: u32,
    pub paid: u64,
    pub acc: u32,
}

#[derive(Accounts)]
pub struct Config<'info> {
    #[account(init, payer = host, space = 8 + 32 + 8 + 4 + 8 + 4)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub host: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct RecordAndPay<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    pub host: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub fund: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub winner_ata: UncheckedAccount<'info>,
    /// CHECK:
    pub token_program: UncheckedAccount<'info>,
}
#[error_code]
pub enum Errs { #[msg("host mismatch")] Host }
