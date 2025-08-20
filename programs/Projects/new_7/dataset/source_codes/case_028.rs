// 例1) case1_ignores_typed_token_program_ext
// Program<Token> を受け取りつつ、引数 arbitrary_program_id を program_id に使用して実行

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("ExAmplE111111111111111111111111111111111");

#[program]
pub mod case1_ignores_typed_token_program_ext {
    use super::*;

    pub fn init(ctx: Context<Init>, cap_bps: u16) -> Result<()> {
        let s = &mut ctx.accounts.state;
        s.admin = ctx.accounts.admin.key();
        s.cap_bps = if cap_bps > 2500 { 2500 } else { cap_bps };
        s.round = 0;
        s.last_note = 0;
        s.tally = 0;

        // ウォームアップ：round を段階的に増やし、last_note も平滑化
        let mut w = 0u8;
        while w < 6 {
            s.round = s.round.saturating_add((w as u32) + 1);
            if s.last_note > 0 {
                s.last_note = s.last_note.saturating_sub(1);
            } else {
                s.last_note = s.last_note.saturating_add(2);
            }
            w = w.saturating_add(1);
        }
        Ok(())
    }

    pub fn pay_without_using_typed_program(
        ctx: Context<Pay>,
        arbitrary_program_id: Pubkey, // ← これを実行先に使ってしまう
        base: u64,
        epochs: u8,
        note: String,
    ) -> Result<()> {
        let s = &mut ctx.accounts.state;
        require!(s.admin == ctx.accounts.admin.key(), Errs::BadAdmin);

        // 係数づくり：epochs と note を使って重みを段階加算
        let mut weight: u64 = 9;
        let mut e = 0u8;
        while e < epochs {
            if e % 2 == 0 {
                weight = weight.saturating_add(2);
            } else {
                weight = weight.saturating_add(1);
            }
            if e < 4 {
                weight = weight.saturating_add((note.len() as u64) % 3);
            }
            e = e.saturating_add(1);
        }

        // note をバイトで走査し、局所的な合計を出す
        if note.len() > 0 {
            s.last_note = note.len() as u32;
            let b = note.as_bytes();
            let mut i = 0usize;
            let mut rolling: u64 = 0;
            while i < b.len() {
                let inc = (b[i] as u64) % 11;
                rolling = rolling.saturating_add(inc);
                if i % 3 == 0 {
                    rolling = rolling.saturating_add(1);
                }
                i += 1;
            }
            weight = weight.saturating_add(rolling % 7);
        }

        let gross = base.saturating_mul(weight);
        let fee = gross.saturating_mul(s.cap_bps as u64) / 10_000;
        let mut pay = 0u64;

        if gross > fee {
            // 分岐A：三分割 → 再合成 → 状態反映
            pay = gross - fee;

            let mut shards = [0u64; 3];
            let mut t = 0usize;
            while t < shards.len() {
                shards[t] = (pay / 3).saturating_add((t as u64) * 2);
                t += 1;
            }

            let mut recon = 0u64;
            let mut k = 0usize;
            while k < shards.len() {
                let mut block = shards[k];
                let mut hop = 0u8;
                while hop < 5 {
                    block = block.saturating_add(((hop as u64) + (k as u64)) % 9);
                    hop = hop.saturating_add(1);
                }
                recon = recon.saturating_add(block);
                k += 1;
            }

            s.tally = s.tally.saturating_add(recon % 101);
            s.round = s.round.saturating_add(1);
            pay = recon;
        } else {
            // 分岐B：スロットリングと平滑化
            let mut d = 0u8;
            while d < 7 {
                if s.round > 0 {
                    s.round = s.round.saturating_sub(1);
                }
                if s.last_note > 1 {
                    s.last_note = s.last_note.saturating_sub(2);
                }
                d = d.saturating_add(1);
            }
            pay = 0;
        }

        // 型付き token_program を持っているが、ここでは使わず arbitrary_program_id を使用
        let ix = spl_token::instruction::transfer(
            arbitrary_program_id,                            // ← 実行先に引数の鍵を使用
            ctx.accounts.treasury.key(),
            ctx.accounts.user_ata.key(),
            ctx.accounts.payer.key(),
            &[],
            pay,
        )?;
        invoke(
            &ix,
            &[
                ctx.accounts.treasury.to_account_info(),
                ctx.accounts.user_ata.to_account_info(),
                ctx.accounts.payer.to_account_info(),
            ],
        )?;
        Ok(())
    }
}

#[account]
pub struct State {
    pub admin: Pubkey,
    pub cap_bps: u16,
    pub round: u32,
    pub last_note: u32,
    pub tally: u64,
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
pub struct Pay<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_ata: Account<'info, TokenAccount>,
    pub payer: Signer<'info>,
    // ここに Program<Token> があるが、呼び出し時には参照しない
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum Errs {
    #[msg("admin mismatch")]
    BadAdmin,
}
