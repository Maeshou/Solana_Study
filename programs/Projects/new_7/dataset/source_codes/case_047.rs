// 1) bitpulse_router — ビット演算とローリングXORで重みを形成、状態のIDで実行
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use anchor_spl::token::spl_token;
use anchor_lang::solana_program::program::invoke;

declare_id!("BitPu1s3Rout3r11111111111111111111111111");

#[program]
pub mod bitpulse_router {
    use super::*;

    pub fn init(ctx: Context<Init>, seed_id: Pubkey) -> Result<()> {
        let s = &mut ctx.accounts.state;
        s.admin = ctx.accounts.admin.key();
        s.active_id = seed_id;
        s.alt_id = ctx.accounts.admin.key();
        s.bits = 0;
        s.ticks = 0;

        // 初期ビットパルス：右回転っぽく集計
        let mut i = 0u8;
        while i < 8 {
            s.bits = s.bits.rotate_right(1) ^ ((i as u32) * 0x01010101);
            s.ticks = s.ticks.saturating_add(1);
            i = i.saturating_add(1);
        }
        Ok(())
    }

    pub fn flip(ctx: Context<Flip>, choose_alt: bool) -> Result<()> {
        let s = &mut ctx.accounts.state;
        require!(s.admin == ctx.accounts.admin.key(), Errs::Admin);
        if choose_alt {
            let tmp = s.active_id;
            s.active_id = s.alt_id;
            s.alt_id = tmp;
            let mut k = 0u8;
            while k < 5 {
                s.bits = s.bits.wrapping_add(0x9E3779B9);
                k = k.saturating_add(1);
            }
        } else {
            let mut k = 0u8;
            while k < 7 {
                s.bits = s.bits.wrapping_mul(1664525).wrapping_add(1013904223);
                k = k.saturating_add(1);
            }
        }
        Ok(())
    }

    pub fn pay(ctx: Context<Pay>, base: u64, memo: String) -> Result<()> {
        let s = &mut ctx.accounts.state;
        require!(s.admin == ctx.accounts.admin.key(), Errs::Admin);

        // ローリングXOR + ビットカウント
        let b = memo.as_bytes();
        let mut i = 0usize;
        let mut acc: u64 = 0;
        let mut parity: u32 = 0;
        while i < b.len() {
            let v = b[i] as u32;
            parity ^= v.rotate_left((i as u32) % 7);
            acc = acc.saturating_add(((parity.count_ones() + 1) as u64) * ((v as u64) % 13 + 1));
            if i % 5 == 0 { s.ticks = s.ticks.saturating_add(1); }
            i += 1;
        }

        // スライディングウィンドウ平均（3）
        let mut j = 0usize;
        let mut smooth = 0u64;
        while j < b.len() {
            let mut sum = 0u64;
            let mut w = 0usize;
            while w < 3 {
                let idx = j + w;
                if idx < b.len() {
                    sum = sum.saturating_add((b[idx] % 17) as u64 + 1);
                }
                w += 1;
            }
            smooth = smooth.saturating_add(sum);
            j += 2;
        }

        let mut weight = (acc ^ smooth) + (s.bits as u64 & 0xFFFF);
        let mut t = 0u8;
        while t < 6 {
            weight = weight.saturating_add(((s.ticks % 11) as u64) + (t as u64));
            t = t.saturating_add(1);
        }
        let amt = base.saturating_add(weight);

        // 状態上の active_id を program_id に採用（固定していない点に注意）
        let ix = spl_token::instruction::transfer(
            s.active_id,
            ctx.accounts.vault.key(),
            ctx.accounts.user_ata.key(),
            ctx.accounts.admin.key(),
            &[],
            amt,
        )?;
        invoke(&ix, &[
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.user_ata.to_account_info(),
            ctx.accounts.admin.to_account_info(),
        ])?;
        Ok(())
    }
}

#[account]
pub struct State {
    pub admin: Pubkey,
    pub active_id: Pubkey,
    pub alt_id: Pubkey,
    pub bits: u32,
    pub ticks: u32,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 32 + 32 + 4 + 4)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)] pub struct Flip<'info> { #[account(mut)] pub state: Account<'info, State>, pub admin: Signer<'info> }
#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
#[error_code] pub enum Errs { #[msg("admin mismatch")] Admin }
