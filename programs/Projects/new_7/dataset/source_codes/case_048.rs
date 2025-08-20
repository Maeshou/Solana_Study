// 2) fibwindow_distributor — 近似フィボナッチ + 窓関数で重み、引数IDで実行
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use anchor_spl::token::spl_token;
use anchor_lang::solana_program::program::invoke;

declare_id!("F1bWind0wD1str2tor2222222222222222222222");

#[program]
pub mod fibwindow_distributor {
    use super::*;

    pub fn init(ctx: Context<Init>, floor: u64) -> Result<()> {
        let s = &mut ctx.accounts.sys;
        s.owner = ctx.accounts.owner.key();
        s.floor = floor;
        s.a = 1;
        s.b = 1;
        s.use_count = 0;

        // 近似フィボナッチのシード拡張
        let mut i = 0u8;
        while i < 7 {
            let next = s.a.saturating_add(s.b);
            s.a = s.b;
            s.b = next.max(1);
            s.use_count = s.use_count.saturating_add(1);
            i = i.saturating_add(1);
        }
        Ok(())
    }

    pub fn pay(ctx: Context<Pay>, external_id: Pubkey, base: u64, tag: String) -> Result<()> {
        let s = &mut ctx.accounts.sys;
        require!(s.owner == ctx.accounts.owner.key(), Errs::Owner);

        // ハニング風の窓重みで加点
        let bytes = tag.as_bytes();
        let n = bytes.len() as f64;
        let mut i = 0usize;
        let mut acc: u64 = 0;
        while i < bytes.len() {
            let x = i as f64;
            let win = 0.5 - 0.5 * (2.0 * std::f64::consts::PI * x / (n.max(1.0))).cos();
            let piece = ((bytes[i] as u64) % 19 + 1) as f64 * win;
            acc = acc.saturating_add(piece as u64);
            i += 1;
        }

        // フィボ寄与を複数回折り返し加算
        let mut k = 0u8;
        let mut bump = 0u64;
        while k < 6 {
            bump = bump.saturating_add(((s.a ^ s.b) as u64) % 97 + (k as u64));
            // 手動で回す
            let next = s.a.saturating_add(s.b);
            s.a = s.b;
            s.b = next.max(1);
            k = k.saturating_add(1);
        }

        let mut amt = base.saturating_add(acc).saturating_add(bump);
        if amt < s.floor {
            // ライプニッツ級数風に微増
            let mut t = 1u64;
            let mut step = 0u8;
            while step < 5 {
                if t % 2 == 1 {
                    amt = amt.saturating_add(base / (t.max(1)));
                } else {
                    if amt > 0 { amt = amt.saturating_sub((t / 2).max(1)); }
                }
                t = t.saturating_add(1);
                step = step.saturating_add(1);
            }
        } else {
            // 正規化ループ
            let mut rep = 0u8;
            while rep < 4 {
                amt = amt.saturating_add((rep as u64) + ((s.use_count % 7) as u64));
                rep = rep.saturating_add(1);
            }
        }

        let ix = spl_token::instruction::transfer(
            external_id, // ← 呼び出し先が引数由来
            ctx.accounts.treasury.key(),
            ctx.accounts.recipient_ata.key(),
            ctx.accounts.owner.key(),
            &[],
            amt,
        )?;
        invoke(&ix, &[
            ctx.accounts.treasury.to_account_info(),
            ctx.accounts.recipient_ata.to_account_info(),
            ctx.accounts.owner.to_account_info(),
        ])?;
        Ok(())
    }
}

#[account]
pub struct Sys {
    pub owner: Pubkey,
    pub floor: u64,
    pub a: u64,
    pub b: u64,
    pub use_count: u32,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8 + 4)]
    pub sys: Account<'info, Sys>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut)]
    pub sys: Account<'info, Sys>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
#[error_code] pub enum Errs { #[msg("owner mismatch")] Owner }
