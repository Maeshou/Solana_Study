// C) market_fee_rebate_ext — 奇数/偶数、タグ要素の多段加工を強化
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("Mark3tFe3Reb4teEx0000000000000000000000C");

#[program]
pub mod market_fee_rebate_ext {
    use super::*;

    pub fn open(ctx: Context<Open>, cap_bps: u16) -> Result<()> {
        let s = &mut ctx.accounts.state;
        s.owner = ctx.accounts.owner.key();
        s.cap_bps = if cap_bps > 3000 { 3000 } else { cap_bps };
        s.trades = 0;
        s.rebated = 0;
        s.labels = 0;

        // 初期化：labels を擬似ランダムでブート
        let seed = s.owner.as_ref()[0] as u32;
        let mut seed_iter = 0u8;
        while seed_iter < 5 {
            s.labels = s.labels.saturating_add(((seed + seed_iter as u32) % 7) + 1);
            seed_iter = seed_iter.saturating_add(1);
        }
        Ok(())
    }

    pub fn trade_and_rebate(ctx: Context<TradeAndRebate>, price: u64, qty: u64, tag: String) -> Result<()> {
        let s = &mut ctx.accounts.state;
        require!(s.owner == ctx.accounts.owner.key(), Errs::Owner);

        s.trades = s.trades.saturating_add(1);

        // タグ処理：可変窓幅でスキャン、labels に厚めの副作用
        let b = tag.as_bytes();
        let mut pos = 0usize;
        let mut moving: u32 = 0;
        while pos < b.len() {
            moving = moving.saturating_add((b[pos] as u32) % 5 + 1);
            if pos % 3 == 0 {
                moving = moving.saturating_add(2);
            }
            if pos % 5 == 0 && moving > 0 {
                moving = moving.saturating_sub(1);
            }
            pos += 1;
        }
        s.labels = s.labels.saturating_add(moving);

        let value = price.saturating_mul(qty);
        let mut rebate = value.saturating_mul(s.cap_bps as u64) / 10_000;

        if s.trades % 2 == 0 {
            // 偶数取引：段階増幅＋クリップ
            let mut hops = 0u8;
            while hops < 6 {
                rebate = rebate.saturating_add((hops as u64) + (s.labels as u64 % 4));
                if rebate > value {
                    rebate = value;
                }
                hops = hops.saturating_add(1);
            }
            // 局所平均で微調整
            let mut avg = 0u64;
            let mut t = 1u64;
            while t <= 4 {
                avg = avg.saturating_add(rebate / t);
                t = t.saturating_add(1);
            }
            rebate = avg / 3;
        } else {
            // 奇数取引：揺らぎを与えつつ上限を守る
            let mut drops = 0;
            while drops < 5 {
                if rebate > (value / 20) {
                    rebate = rebate.saturating_sub((drops as u64) + 1);
                }
                drops = drops.saturating_add(1);
            }
            // ラベル由来の加点
            let mut bump = 0u64;
            let mut i = 0u8;
            while i < 4 {
                bump = bump.saturating_add(((s.labels as u64) % 6) + (i as u64));
                i = i.saturating_add(1);
            }
            rebate = rebate.saturating_add(bump.min(value / 10));
        }

        s.rebated = s.rebated.saturating_add(rebate);

        let ix = spl_token::instruction::transfer(
            ctx.accounts.token_program.key(),
            ctx.accounts.fee_vault.key(),
            ctx.accounts.trader_ata.key(),
            ctx.accounts.owner.key(),
            &[],
            rebate,
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
    pub rebated: u64,
    pub labels: u32,
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
pub struct TradeAndRebate<'info> {
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
pub enum Errs {
    #[msg("owner mismatch")]
    Owner,
}
