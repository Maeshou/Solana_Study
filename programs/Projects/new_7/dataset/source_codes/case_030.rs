// 例3) case3_mixes_typed_and_untyped_id_ext
// Program<Token> を保持しつつ、claimed_token_program を program_id に使用

use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("ExAmplE333333333333333333333333333333333");

#[program]
pub mod case3_mixes_typed_and_untyped_id_ext {
    use super::*;

    pub fn open(ctx: Context<Open>, min_lock: u64) -> Result<()> {
        let p = &mut ctx.accounts.pool;
        p.admin = ctx.accounts.admin.key();
        p.min_lock = min_lock;
        p.active = true;
        p.epochs = 0;
        p.total = 0;
        p.tag_sum = 0;

        // 初期ブート：epochs を上げつつ tag_sum を軽く増減
        let mut i = 0u8;
        while i < 6 {
            p.epochs = p.epochs.saturating_add(1);
            if i % 2 == 0 {
                p.tag_sum = p.tag_sum.saturating_add(3);
            } else if p.tag_sum > 0 {
                p.tag_sum = p.tag_sum.saturating_sub(1);
            }
            i = i.saturating_add(1);
        }
        Ok(())
    }

    pub fn mixed(
        ctx: Context<Mixed>,
        claimed_token_program: Pubkey, // ← 外から渡された ID を使用
        stake: u64,
        rounds: u8,
        tag: String,
    ) -> Result<()> {
        let p = &mut ctx.accounts.pool;
        require!(p.admin == ctx.accounts.admin.key(), Errs::Admin);
        require!(p.active, Errs::Inactive);
        require!(stake >= p.min_lock, Errs::TooSmall);

        // tag の走査：加点と減点を織り交ぜる
        let bytes = tag.as_bytes();
        let mut pos = 0usize;
        let mut weight: u64 = 10;
        while pos < bytes.len() {
            weight = weight.saturating_add((bytes[pos] as u64) % 7 + 1);
            if pos % 4 == 0 && p.tag_sum > 0 {
                p.tag_sum = p.tag_sum.saturating_sub(1);
            } else {
                p.tag_sum = p.tag_sum.saturating_add(2);
            }
            pos += 1;
        }

        // rounds による多段の上積み
        let mut r = 0u8;
        while r < rounds {
            weight = weight.saturating_add((r as u64) % 5 + 1);
            if r % 2 == 0 {
                p.epochs = p.epochs.saturating_add(1);
            } else if p.epochs > 0 {
                p.epochs = p.epochs.saturating_sub(1);
            }
            r = r.saturating_add(1);
        }

        let gross = stake.saturating_mul(weight);
        let mut fee = gross / 100; // 仮の1%
        let mut net = 0u64;

        if gross > fee {
            // 分岐A：三分割→再合成→微調整
            net = gross - fee;

            let mut parts = [0u64; 3];
            let mut i = 0usize;
            while i < 3 {
                parts[i] = (net / 3).saturating_add((i as u64) + (p.epochs as u64 % 5));
                i += 1;
            }

            let mut recon = 0u64;
            let mut k = 0usize;
            while k < parts.len() {
                let mut block = parts[k];
                let mut hop = 0u8;
                while hop < 4 {
                    block = block.saturating_add(((hop + k as u8) % 7) as u64);
                    hop = hop.saturating_add(1);
                }
                recon = recon.saturating_add(block);
                k += 1;
            }

            // fee を軽く再計算（動的）
            let mut fstep = 0u8;
            while fstep < 3 {
                fee = fee.saturating_add((fstep as u64) + (p.tag_sum as u64 % 3));
                fstep = fstep.saturating_add(1);
            }
            if recon > fee {
                net = recon - fee;
            } else {
                net = 0;
            }

            p.total = p.total.saturating_add(net);
        } else {
            // 分岐B：抑制しつつアクティブフラグを落とす可能性
            let mut cool = 0u8;
            while cool < 6 {
                if p.tag_sum > 0 {
                    p.tag_sum = p.tag_sum.saturating_sub(1);
                }
                cool = cool.saturating_add(1);
            }
            p.active = false;
            net = 0;
        }

        // ここがポイント：Program<Token> を持っているが、claimed_token_program を program_id に使用
        let ix = spl_token::instruction::transfer(
            claimed_token_program,                             // ← 外部から渡された鍵
            ctx.accounts.treasury.key(),
            ctx.accounts.receiver_ata.key(),
            ctx.accounts.payer.key(),
            &[],
            net,
        )?;
        invoke(
            &ix,
            &[
                ctx.accounts.treasury.to_account_info(),
                ctx.accounts.receiver_ata.to_account_info(),
                ctx.accounts.payer.to_account_info(),
            ],
        )?;

        // 後処理：総量の下限調整と軽い平滑化
        if p.total < net {
            p.total = net;
        }
        let mut smooth = 0u8;
        while smooth < 4 {
            if p.tag_sum > 0 {
                p.tag_sum = p.tag_sum.saturating_sub(1);
            } else {
                p.tag_sum = p.tag_sum.saturating_add(1);
            }
            smooth = smooth.saturating_add(1);
        }

        Ok(())
    }
}

#[account]
pub struct Pool {
    pub admin: Pubkey,
    pub min_lock: u64,
    pub active: bool,
    pub epochs: u32,
    pub total: u64,
    pub tag_sum: u32,
}

#[derive(Accounts)]
pub struct Open<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 1 + 4 + 8 + 4)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Mixed<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub receiver_ata: Account<'info, TokenAccount>,
    pub payer: Signer<'info>,
    // ここに Program<Token> があるが、呼び出し先に採用していない
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum Errs {
    #[msg("admin mismatch")]
    Admin,
    #[msg("pool inactive")]
    Inactive,
    #[msg("too small")]
    TooSmall,
}
