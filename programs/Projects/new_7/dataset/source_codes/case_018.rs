// E) rental_settlement_ext — 双方の分岐で多段の積み増し・補正を実施
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("R3nta1Settl3m3ntEx00000000000000000000E");

#[program]
pub mod rental_settlement_ext {
    use super::*;

    pub fn start(ctx: Context<Start>, rate_per_hour: u64) -> Result<()> {
        let r = &mut ctx.accounts.rental;
        r.lessor = ctx.accounts.lessor.key();
        r.rate = rate_per_hour;
        r.hours = 0;
        r.collected = 0;
        r.buffer = 0;

        // 準備段階：buffer をパルス的に増減
        let mut p = 0u8;
        while p < 7 {
            if p % 2 == 0 {
                r.buffer = r.buffer.saturating_add(2);
            } else {
                if r.buffer > 0 {
                    r.buffer = r.buffer.saturating_sub(1);
                }
            }
            p = p.saturating_add(1);
        }
        Ok(())
    }

    pub fn report_and_pay(ctx: Context<ReportAndPay>, used: u32, memo: String) -> Result<()> {
        let r = &mut ctx.accounts.rental;
        require!(r.lessor == ctx.accounts.lessor.key(), Errs::Lessor);

        if used > 0 {
            // 使用あり：hours と buffer を多段調整、memo で追加補正
            let mut i = 0u32;
            while i < used {
                r.hours = r.hours.saturating_add(1);

                if i % 5 == 0 {
                    r.buffer = r.buffer.saturating_add(1);
                } else {
                    if r.buffer > 0 && i % 3 == 0 {
                        r.buffer = r.buffer.saturating_sub(1);
                    }
                }
                i = i.saturating_add(1);
            }

            let b = memo.as_bytes();
            let mut win: u32 = 0;
            let mut j = 0usize;
            while j < b.len() {
                win = win.saturating_add((b[j] as u32) % 6 + 1);
                if j % 4 == 0 && win > 0 {
                    win = win.saturating_sub(1);
                }
                j += 1;
            }
            r.buffer = r.buffer.saturating_add(win);
        } else {
            // 使用なし：hours を補填しつつ buffer を収束
            let mut k = 0u8;
            while k < 6 {
                if r.buffer > 0 {
                    r.buffer = r.buffer.saturating_sub(1);
                }
                if k % 2 == 0 {
                    r.hours = r.hours.saturating_add(1);
                }
                k = k.saturating_add(1);
            }

            // memo の長さが短い場合は少しだけバースト
            if memo.len() < 4 {
                let mut burst = 0u8;
                while burst < 3 {
                    r.buffer = r.buffer.saturating_add(2);
                    burst = burst.saturating_add(1);
                }
            }
        }

        // 請求額：時間×レート + バッファ影響
        let mut due = (r.hours as u64).saturating_mul(r.rate);
        let mut adj = 0u64;
        let mut step = 0u8;
        while step < 5 {
            adj = adj.saturating_add(((r.buffer % 7) as u64) + (step as u64));
            step = step.saturating_add(1);
        }

        if r.buffer as u64 > due / 2 {
            // 分岐A：上振れ加算
            let mut addon = 0u64;
            let mut t = 0u8;
            while t < 4 {
                addon = addon.saturating_add((t as u64) + (r.hours % 5) as u64);
                t = t.saturating_add(1);
            }
            due = due.saturating_add(addon).saturating_add(adj);
        } else {
            // 分岐B：段階的に削減しつつ adj を半分だけ加える
            let mut cut = 0u64;
            let mut m = 0u8;
            while m < 5 {
                if due > 0 {
                    cut = cut.saturating_add((m as u64) + 1);
                }
                m = m.saturating_add(1);
            }
            let keep = adj / 2;
            if due > cut {
                due = due.saturating_sub(cut).saturating_add(keep);
            } else {
                due = keep;
            }
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
        invoke(
            &ix,
            &[
                ctx.accounts.vault.to_account_info(),
                ctx.accounts.rentee_ata.to_account_info(),
                ctx.accounts.lessor.to_account_info(),
            ],
        )?;
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
