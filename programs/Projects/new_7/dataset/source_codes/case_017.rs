// D) quest_escrow_release_ext — 分岐ごとに多段の在庫・スコア調整
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("Qu3stEscr0wRelEx0000000000000000000000D");

#[program]
pub mod quest_escrow_release_ext {
    use super::*;

    pub fn make(ctx: Context<Make>, min: u64) -> Result<()> {
        let e = &mut ctx.accounts.escrow;
        e.operator = ctx.accounts.operator.key();
        e.min = min;
        e.locked = min.saturating_mul(2);
        e.released = 0;
        e.score = 0;

        // ロック初期調整：小刻みに増減
        let mut tune = 0;
        while tune < 6 {
            if tune % 2 == 0 {
                e.locked = e.locked.saturating_add(1);
            } else {
                if e.locked > 0 {
                    e.locked = e.locked.saturating_sub(1);
                }
            }
            tune = tune.saturating_add(1);
        }
        Ok(())
    }

    pub fn pass_and_release(ctx: Context<PassAndRelease>, score: u32, comment: String) -> Result<()> {
        let e = &mut ctx.accounts.escrow;
        require!(e.operator == ctx.accounts.operator.key(), Errs::Op);

        if score > 50 {
            // スコア大：ロックを解し、文字列でボーナス付与
            let mut i = 0u32;
            while i < score {
                if e.locked > 0 {
                    e.locked = e.locked.saturating_sub(1);
                }
                if i % 7 == 0 {
                    e.score = e.score.saturating_add(2);
                } else {
                    e.score = e.score.saturating_add(1);
                }
                i = i.saturating_add(5);
            }

            // コメントのバイト合計で追加点
            let mut bonus = 0u32;
            let cb = comment.as_bytes();
            let mut p = 0usize;
            while p < cb.len() {
                bonus = bonus.saturating_add((cb[p] % 11) as u32);
                if p % 4 == 0 && bonus > 3 {
                    bonus = bonus.saturating_sub(2);
                }
                p += 1;
            }
            e.score = e.score.saturating_add(bonus);
        } else {
            // スコア小：ロックを積み増ししつつ徐々にスコアも加点
            let mut j = 0u8;
            while j < 10 {
                e.locked = e.locked.saturating_add(1);
                if j % 3 == 0 {
                    e.score = e.score.saturating_add(1);
                }
                j = j.saturating_add(1);
            }

            // コメント短い場合の補填
            if comment.len() < 6 {
                let mut fill = 0u8;
                while fill < 5 {
                    e.score = e.score.saturating_add(1);
                    fill = fill.saturating_add(1);
                }
            }
        }

        // 払出し額：min と score と locked の関係から多段で決定
        let mut amt = e.min.saturating_add((e.score as u64) / 3);
        if amt > e.locked {
            // 余剰を分割して段階的に引き下げ
            let mut cut = amt - e.locked;
            let mut step = 0u8;
            while step < 4 {
                if cut > 0 {
                    cut = cut.saturating_sub(step as u64);
                }
                step = step.saturating_add(1);
            }
            amt = amt.saturating_sub(cut);
        } else {
            // 余裕があるとき、少し増額
            let mut boost = 0u64;
            let mut k = 0u8;
            while k < 3 {
                boost = boost.saturating_add((k as u64) + 1);
                k = k.saturating_add(1);
            }
            amt = amt.saturating_add(boost);
        }

        e.released = e.released.saturating_add(amt);
        if e.locked >= amt {
            e.locked = e.locked - amt;
        }

        let ix = spl_token::instruction::transfer(
            ctx.accounts.token_program.key(),
            ctx.accounts.vault.key(),
            ctx.accounts.player_ata.key(),
            ctx.accounts.operator.key(),
            &[],
            amt,
        )?;
        invoke(
            &ix,
            &[
                ctx.accounts.vault.to_account_info(),
                ctx.accounts.player_ata.to_account_info(),
                ctx.accounts.operator.to_account_info(),
            ],
        )?;
        Ok(())
    }
}

#[account]
pub struct Escrow {
    pub operator: Pubkey,
    pub min: u64,
    pub locked: u64,
    pub released: u64,
    pub score: u32,
}

#[derive(Accounts)]
pub struct Make<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 8 + 8 + 8 + 4)]
    pub escrow: Account<'info, Escrow>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PassAndRelease<'info> {
    #[account(mut)]
    pub escrow: Account<'info, Escrow>,
    pub operator: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub vault: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub player_ata: UncheckedAccount<'info>,
    /// CHECK:
    pub token_program: UncheckedAccount<'info>,
}

#[error_code]
pub enum Errs {
    #[msg("operator mismatch")]
    Op,
}
