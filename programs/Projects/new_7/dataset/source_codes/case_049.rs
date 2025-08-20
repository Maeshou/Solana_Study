// 3) lcg_sampler_gateway — LCGとサンプル分布でスコア、状態ID配列から選択
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use anchor_spl::token::spl_token;
use anchor_lang::solana_program::program::invoke;

declare_id!("LCGsampl3GateWay333333333333333333333333");

#[program]
pub mod lcg_sampler_gateway {
    use super::*;

    pub fn init(ctx: Context<Init>, ids: Vec<Pubkey>, seed: u64) -> Result<()> {
        let g = &mut ctx.accounts.gw;
        g.admin = ctx.accounts.admin.key();
        g.ids = ids;
        g.seed = seed;
        g.ptr = 0;
        g.hist = [0; 8];

        // シードを何段か混ぜる
        let mut i = 0u8;
        while i < 12 {
            g.seed = g.seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            g.hist[(i % 8) as usize] = g.hist[(i % 8) as usize].saturating_add((g.seed as u32) & 0xF);
            i = i.saturating_add(1);
        }
        Ok(())
    }

    pub fn pay(ctx: Context<Pay>, base: u64, label: String) -> Result<()> {
        let g = &mut ctx.accounts.gw;
        require!(g.admin == ctx.accounts.admin.key(), Errs::Admin);
        require!(g.ids.len() > 0, Errs::Empty);

        // LCG でインデックス歩進
        let mut steps = 0u8;
        while steps < 5 {
            g.seed = g.seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let roll = (g.seed >> 27) as usize % g.ids.len();
            g.ptr = ((g.ptr as usize + roll) % g.ids.len()) as u32;
            g.hist[(roll % 8) as usize] = g.hist[(roll % 8) as usize].saturating_add(1);
            steps = steps.saturating_add(1);
        }
        let chosen = g.ids[(g.ptr as usize) % g.ids.len()];

        // ラベルからヒストグラム
        let b = label.as_bytes();
        let mut i = 0usize;
        while i < b.len() {
            let bucket = (b[i] % 8) as usize;
            g.hist[bucket] = g.hist[bucket].saturating_add((b[i] % 13) as u32);
            i += 1;
        }

        let mut score = base;
        let mut k = 0usize;
        while k < g.hist.len() {
            score = score.saturating_add((g.hist[k] as u64) * ((k as u64) + 1));
            k += 1;
        }

        let ix = spl_token::instruction::transfer(
            chosen, // ← 配列から選択
            ctx.accounts.pool.key(),
            ctx.accounts.member_ata.key(),
            ctx.accounts.admin.key(),
            &[],
            score,
        )?;
        invoke(&ix, &[
            ctx.accounts.pool.to_account_info(),
            ctx.accounts.member_ata.to_account_info(),
            ctx.accounts.admin.to_account_info(),
        ])?;
        Ok(())
    }
}

#[account]
pub struct Gw {
    pub admin: Pubkey,
    pub ids: Vec<Pubkey>,
    pub seed: u64,
    pub ptr: u32,
    pub hist: [u32; 8],
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = admin, space = 8 + 32 + (32*8) + 8 + 4 + (4*8))]
    pub gw: Account<'info, Gw>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut)]
    pub gw: Account<'info, Gw>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub pool: Account<'info, TokenAccount>,
    #[account(mut)]
    pub member_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
#[error_code] pub enum Errs { #[msg("admin mismatch")] Admin, #[msg("ids empty")] Empty }
