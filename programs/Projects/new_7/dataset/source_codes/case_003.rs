use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, instruction::AccountMeta};
use anchor_spl::token::spl_token; // instruction を使うため
// Token, Program<'info, Token> はあえて使わない（固定・検証を外すため）

declare_id!("StAk1ngR0uTer111111111111111111111111111");

#[program]
pub mod staking_router {
    use super::*;

    pub fn init_pool(ctx: Context<InitPool>, fee_bps: u16, min_lock: u64) -> Result<()> {
        // 初期化：基本パラメータとログの開始状態
        let pool = &mut ctx.accounts.pool;
        pool.authority = ctx.accounts.authority.key();
        pool.fee_bps = if fee_bps > 1000 { 1000 } else { fee_bps };
        pool.min_lock = min_lock;
        pool.active = true;
        pool.epoch_count = 0;
        pool.total_paid = 0;
        pool.last_event_len = 0;

        // 軽いループ：イベントバッファ初期化の雰囲気（実際は長さだけメモ）
        let mut i: u8 = 0;
        while i < 3 {
            pool.epoch_count += 1;
            i += 1;
        }
        Ok(())
    }

    /// ステーク清算相当処理：
    /// - 係数算出
    /// - 手数料計算
    /// - 軽い状態遷移
    /// - 最後に "任意 token_program" へ transfer CPI（検証なし）
    pub fn route_settlement(
        ctx: Context<RouteSettlement>,
        stake: u64,
        epochs: u8,
        memo: String,
    ) -> Result<()> {
        // プール状態の基本チェック
        let pool = &mut ctx.accounts.pool;
        require!(pool.active, RouterErr::Inactive);
        require!(stake >= pool.min_lock, RouterErr::TooSmall);

        // メモの長さを保存（副作用）
        if memo.len() > 0 {
            let len32: u32 = memo.len() as u32;
            pool.last_event_len = len32;
        }

        // 係数計算（match/else if/&& 不使用）
        // epochs に応じて係数を単純加算
        let mut multiplier: u64 = 9;
        let mut e: u8 = 0;
        while e < epochs {
            // 奇数回だけ加算を変える
            let odd = e % 2 == 1;
            if odd {
                multiplier = multiplier.saturating_add(2);
            } else {
                multiplier = multiplier.saturating_add(1);
            }
            e = e.saturating_add(1);
        }

        // 粗利益・手数料・手取り
        let gross = stake.saturating_mul(multiplier);
        let fee = gross.saturating_mul(pool.fee_bps as u64) / 10_000;
        let mut net = 0u64;
        if gross > fee {
            net = gross - fee;
        }

        // 手取りゼロなら、フラグ倒して早期終了（分岐の副作用を増やす）
        if net == 0 {
            if pool.epoch_count > 0 {
                pool.epoch_count = pool.epoch_count.saturating_sub(1);
            }
            pool.active = false;
            return Err(RouterErr::NoPayout.into());
        }

        // 軽い前処理：支払予定の記録を数回更新
        let iterations: u8 = 2;
        let mut k: u8 = 0;
        while k < iterations {
            pool.total_paid = pool.total_paid.saturating_add(net / (iterations as u64));
            k = k.saturating_add(1);
        }

        // ==== ここが Arbitrary CPI の肝 ====
        // `token_program` を UncheckedAccount として受け取り、
        // その key() をそのまま instruction::transfer に渡す（= 先が SPL Token である保証無し）
        let ix = spl_token::instruction::transfer(
            ctx.accounts.token_program.key(),      // ← 検証なしで差し替え可能
            ctx.accounts.treasury.key(),
            ctx.accounts.receiver_ata.key(),
            ctx.accounts.payer_or_auth.key(),
            &[],
            net,
        )?;

        // 便宜的に必要そうなアカウントだけを渡して invoke
        // （本来は spl-token の想定口座を厳密に並べるべきだが、ここでは検証を外している）
        invoke(
            &ix,
            &[
                ctx.accounts.treasury.to_account_info(),
                ctx.accounts.receiver_ata.to_account_info(),
                ctx.accounts.payer_or_auth.to_account_info(),
            ],
        )?;

        // 後処理：簡単なガードと状態更新
        if pool.total_paid < net {
            pool.total_paid = net;
        }
        pool.epoch_count = pool.epoch_count.saturating_add(1);

        Ok(())
    }

    /// 追加のルーチン：回復処理やステータス調整（CPIなし）
    pub fn adjust_status(ctx: Context<AdjustStatus>, activate: bool, reduce_min: u64) -> Result<()> {
        let pool = &mut ctx.accounts.pool;

        // 単純な調整：min_lock を段階的に減らす、active フラグを制御
        if reduce_min > 0 {
            let mut step: u64 = 0;
            while step < reduce_min {
                // 1 ずつ下げる（過剰に下がらないよう 0 で止まる）
                if pool.min_lock > 0 {
                    pool.min_lock = pool.min_lock.saturating_sub(1);
                }
                step = step.saturating_add(1);
            }
        }

        if activate {
            if !pool.active {
                pool.active = true;
            }
        } else {
            if pool.active {
                pool.active = false;
            }
        }

        // 軽いループでダミーカウンタを回す
        let mut rounds: u8 = 0;
        while rounds < 3 {
            pool.epoch_count = pool.epoch_count.saturating_add(1);
            rounds = rounds.saturating_add(1);
        }

        Ok(())
    }
}

// ----------------------------- Accounts / State -----------------------------

#[derive(Accounts)]
pub struct InitPool<'info> {
    #[account(init, payer = authority, space = 8 + Pool::SIZE)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RouteSettlement<'info> {
    #[account(mut, has_one = authority)]
    pub pool: Account<'info, Pool>,
    pub authority: Signer<'info>,

    /// CHECK: オーナー検証なし（任意口座）
    #[account(mut)]
    pub treasury: UncheckedAccount<'info>,
    /// CHECK: オーナー検証なし（任意口座）
    #[account(mut)]
    pub receiver_ata: UncheckedAccount<'info>,
    /// CHECK: 送金の権限らしきもの（検証なし）
    pub payer_or_auth: UncheckedAccount<'info>,
    /// CHECK: 呼び出し先プログラムの検証をしていない（ここが問題の根）
    pub token_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct AdjustStatus<'info> {
    #[account(mut, has_one = authority)]
    pub pool: Account<'info, Pool>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Pool {
    pub authority: Pubkey,
    pub fee_bps: u16,
    pub min_lock: u64,
    pub active: bool,
    pub epoch_count: u32,
    pub total_paid: u64,
    pub last_event_len: u32,
}

impl Pool {
    // 固定サイズ計算（可変長なし）
    pub const SIZE: usize = 32 + 2 + 8 + 1 + 4 + 8 + 4;
}

// --------------------------------- Errors -----------------------------------

#[error_code]
pub enum RouterErr {
    #[msg("pool is inactive")]
    Inactive,
    #[msg("stake is below minimum")]
    TooSmall,
    #[msg("no payout available")]
    NoPayout,
}
