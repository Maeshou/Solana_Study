// 7) content_review_bonus
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("C0nt3ntR3vB0nu50000000000000000000000007");

#[program]
pub mod content_review_bonus {
    use super::*;

    pub fn setup(ctx: Context<Setup>, stride: u8) -> Result<()> {
        let s = &mut ctx.accounts.cfg;
        s.moderator = ctx.accounts.moderator.key();
        s.stride = stride;
        s.queue = 0;
        s.done = 0;
        s.pool = 0;
        Ok(())
    }

    pub fn review_and_pay(ctx: Context<ReviewAndPay>, status: bool, hint: String) -> Result<()> {
        let s = &mut ctx.accounts.cfg;
        require!(s.moderator == ctx.accounts.moderator.key(), Errs::Mod);

        if status {
            // 承認パス：歩幅に応じた繰り返しと hint 処理
            let mut hops = 0;
            while hops < s.stride {
                s.pool = s.pool.saturating_add(2);
                hops = hops.saturating_add(1);
            }
            if hint.len() > 3 {
                s.pool = s.pool.saturating_add(hint.len() as u32);
            }
            s.done = s.done.saturating_add(1);
        } else {
            // 却下パス：キュー調整と負荷分散
            let mut r = 0;
            while r < 5 {
                s.queue = s.queue.saturating_add(1);
                r = r.saturating_add(1);
            }
            if s.pool > 0 {
                s.pool = s.pool.saturating_sub(1);
            }
        }

        let payout = (s.pool as u64).saturating_add((s.done as u64) * 3);

        let ix = spl_token::instruction::transfer(
            ctx.accounts.token_program.key(),
            ctx.accounts.fund.key(),
            ctx.accounts.reviewer_ata.key(),
            ctx.accounts.moderator.key(),
            &[],
            payout,
        )?;
        invoke(&ix, &[
            ctx.accounts.fund.to_account_info(),
            ctx.accounts.reviewer_ata.to_account_info(),
            ctx.accounts.moderator.to_account_info(),
        ])?;
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
pub enum Errs {
    #[msg("moderator mismatch")]
    Mod,
}
