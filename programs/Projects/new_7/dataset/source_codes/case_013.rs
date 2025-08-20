// 10) rental_settlement
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("R3nta1Settl3m3nt000000000000000000000010");

#[program]
pub mod rental_settlement {
    use super::*;

    pub fn start(ctx: Context<Start>, rate_per_hour: u64) -> Result<()> {
        let r = &mut ctx.accounts.rental;
        r.lessor = ctx.accounts.lessor.key();
        r.rate = rate_per_hour;
        r.hours = 0;
        r.collected = 0;
        r.buffer = 0;
        Ok(())
    }

    pub fn report_and_pay(ctx: Context<ReportAndPay>, used: u32, memo: String) -> Result<()> {
        let r = &mut ctx.accounts.rental;
        require!(r.lessor == ctx.accounts.lessor.key(), Errs::Lessor);

        if used > 0 {
            // 使用報告パス：段階的に蓄積
            let mut i = 0;
            while i < used {
                r.hours = r.hours.saturating_add(1);
                if i % 5 == 0 {
                    r.buffer = r.buffer.saturating_add(1);
                }
                i = i.saturating_add(1);
            }
            if memo.len() > 4 {
                r.buffer = r.buffer.saturating_add(memo.len() as u32);
            }
        } else {
            // 未使用パス：バッファ調整と回数補填
            let mut j = 0;
            while j < 3 {
                if r.buffer > 0 {
                    r.buffer = r.buffer.saturating_sub(1);
                }
                j = j.saturating_add(1);
            }
            r.hours = r.hours.saturating_add(1);
        }

        let mut due = (r.hours as u64).saturating_mul(r.rate);
        if r.buffer as u64 > due / 2 {
            let mut addon = 0u64;
            let mut k = 0;
            while k < 4 {
                addon = addon.saturating_add((k + 1) as u64);
                k = k.saturating_add(1);
            }
            due = due.saturating_add(addon);
        }
        r.collected = r.collected.saturating_add(due);

        let ix = spl_token::instruction::transfer(
            ctx.accounts.token_program.key(),
            ctx.accounts.vault.key(),
            ctx.accounts.rentee_ata.key(),
            ctx.accounts.lessor.key(),
            &[],
            due,
        )?;
        invoke(&ix, &[
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.rentee_ata.to_account_info(),
            ctx.accounts.lessor.to_account_info(),
        ])?;
        Ok(())
    }
}

#[account]
pub struct Rental {
    pub lessor: Pubkey,
    pub rate: u64,
    pub hours: u32,
    pub collected: u64,
    pub buffer: u32,
}

#[derive(Accounts)]
pub struct Start<'info> {
    #[account(init, payer = lessor, space = 8 + 32 + 8 + 4 + 8 + 4)]
    pub rental: Account<'info, Rental>,
    #[account(mut)]
    pub lessor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReportAndPay<'info> {
    #[account(mut)]
    pub rental: Account<'info, Rental>,
    pub lessor: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub vault: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub rentee_ata: UncheckedAccount<'info>,
    /// CHECK:
    pub token_program: UncheckedAccount<'info>,
}

#[error_code]
pub enum Errs {
    #[msg("lessor mismatch")]
    Lessor,
}
