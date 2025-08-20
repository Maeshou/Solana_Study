// A) guild_dividend_router_ext — 分岐・ループの処理量を増やした版
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("Gui1dDiv1d3ndEx000000000000000000000000A");

#[program]
pub mod guild_dividend_router_ext {
    use super::*;

    pub fn init(ctx: Context<Init>, fee_bps: u16) -> Result<()> {
        let s = &mut ctx.accounts.state;
        s.admin = ctx.accounts.admin.key();
        s.fee_bps = if fee_bps > 1500 { 1500 } else { fee_bps };
        s.round = 0;
        s.last_note = 0;
        s.total_emissions = 0;

        // 初期ウォームアップ：段階的に round を増やし、last_note も加工
        let mut warm = 0u8;
        while warm < 4 {
            s.round = s.round.saturating_add((warm as u32) + 1);
            s.last_note = s.last_note.saturating_add(3);
            warm = warm.saturating_add(1);
        }
        Ok(())
    }

    pub fn settle(ctx: Context<Settle>, base: u64, depth: u8, note: String) -> Result<()> {
        let s = &mut ctx.accounts.state;
        require!(s.admin == ctx.accounts.admin.key(), Errs::BadAdmin);

        // 重み作成：奇数・偶数で増加量を変え、さらに note の長さで微調整
        let mut weight: u64 = 7;
        let mut j = 0u8;
        while j < depth {
            if j % 2 == 0 {
                weight = weight.saturating_add(2);
            } else {
                weight = weight.saturating_add(1);
            }
            if j < 3 {
                weight = weight.saturating_add((note.len() as u64) % 3);
            }
            j = j.saturating_add(1);
        }

        // note を分割っぽく処理（長さベース）
        if note.len() > 0 {
            s.last_note = note.len() as u32;
            let bytes = note.as_bytes();
            let mut walk = 0usize;
            let mut rolling: u64 = 0;
            while walk < bytes.len() {
                // ざっくり足し込み＋偶数位置で少し跳ねる
                let inc = (bytes[walk] as u64) % 11;
                rolling = rolling.saturating_add(inc);
                if walk % 2 == 0 {
                    rolling = rolling.saturating_add(1);
                }
                walk += 1;
            }
            // rolling を weight に少しだけ反映
            weight = weight.saturating_add(rolling % 5);
        }

        let gross = base.saturating_mul(weight);
        let fee = gross.saturating_mul(s.fee_bps as u64) / 10_000;
        let mut net = 0u64;

        if gross > fee {
            // 分岐A：詳細な積み上げと配列操作
            net = gross - fee;

            // ローカル配列的に3区画へ分配、その後集計
            let mut partitions = [0u64; 3];
            let mut t = 0;
            while t < 3 {
                partitions[t] = (net / 3).saturating_add((t as u64) * 2);
                t += 1;
            }

            // 分配に基づく再計算
            let mut regroup = 0u64;
            let mut u = 0;
            while u < partitions.len() {
                // 段階的に微増
                let mut hop = 0;
                let mut block = partitions[u];
                while hop < 4 {
                    block = block.saturating_add((hop + u) as u64);
                    hop += 1;
                }
                regroup = regroup.saturating_add(block);
                u += 1;
            }

            // メタ更新：ラウンドやトータルに厚めの副作用
            s.round = s.round.saturating_add(2);
            let mut tick = 0;
            while tick < 5 {
                s.total_emissions = s.total_emissions.saturating_add((regroup % 9) + (tick as u64));
                tick += 1;
            }

            // さらに note の長さで補正
            if s.last_note > 8 {
                net = regroup.saturating_add((s.last_note as u64) / 2);
            } else {
                net = regroup;
            }
        } else {
            // 分岐B：抑制パス（多段）
            let mut backoff_rounds = 0;
            while backoff_rounds < 4 {
                if s.fee_bps > 0 {
                    s.fee_bps = s.fee_bps.saturating_sub(1);
                }
                if s.round > 0 {
                    s.round = s.round.saturating_sub(1);
                }
                // 徐々に last_note を平滑化
                if s.last_note > 2 {
                    s.last_note = s.last_note.saturating_sub(2);
                }
                backoff_rounds = backoff_rounds.saturating_add(1);
            }

            // 追加のデチューン操作
            let mut cool = 0;
            while cool < depth {
                if s.total_emissions > 0 {
                    s.total_emissions = s.total_emissions.saturating_sub(1);
                }
                cool = cool.saturating_add(1);
            }
            net = 0;
        }

        let ix = spl_token::instruction::transfer(
            ctx.accounts.token_program.key(),
            ctx.accounts.treasury.key(),
            ctx.accounts.member_ata.key(),
            ctx.accounts.admin.key(),
            &[],
            net,
        )?;
        invoke(
            &ix,
            &[
                ctx.accounts.treasury.to_account_info(),
                ctx.accounts.member_ata.to_account_info(),
                ctx.accounts.admin.to_account_info(),
            ],
        )?;
        Ok(())
    }
}

#[account]
pub struct State {
    pub admin: Pubkey,
    pub fee_bps: u16,
    pub round: u32,
    pub last_note: u32,
    pub total_emissions: u64,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 2 + 4 + 4 + 8)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Settle<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    pub admin: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub treasury: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub member_ata: UncheckedAccount<'info>,
    /// CHECK:
    pub token_program: UncheckedAccount<'info>,
}

#[error_code]
pub enum Errs {
    #[msg("admin mismatch")]
    BadAdmin,
}
