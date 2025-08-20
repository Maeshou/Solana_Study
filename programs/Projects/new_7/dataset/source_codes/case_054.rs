// 8) chunk_shuffle_stream — チャンク分割・擬似シャッフル、引数IDで実行
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use anchor_spl::token::spl_token;
use anchor_lang::solana_program::program::invoke;

declare_id!("ChunkShuff1eStr3am88888888888888888888888");

#[program]
pub mod chunk_shuffle_stream {
    use super::*;

    pub fn open(ctx: Context<Open>, chunk: u8) -> Result<()> {
        let s = &mut ctx.accounts.stream;
        s.owner = ctx.accounts.owner.key();
        s.chunk = chunk.max(2);
        s.swap_count = 0;
        s.cache = Vec::new();
        Ok(())
    }

    pub fn pay(ctx: Context<Pay>, external_id: Pubkey, payload: String, base: u64) -> Result<()> {
        let s = &mut ctx.accounts.stream;
        require!(s.owner == ctx.accounts.owner.key(), Errs::Owner);

        let b = payload.as_bytes();
        s.cache.clear();

        // チャンク分割
        let mut i = 0usize;
        while i < b.len() {
            let mut sum = 0u64;
            let mut j = 0u8;
            while j < s.chunk {
                let idx = i + j as usize;
                if idx < b.len() {
                    sum = sum.saturating_add((b[idx] % 31) as u64 + 1);
                }
                j = j.saturating_add(1);
            }
            s.cache.push(sum);
            i += s.chunk as usize;
        }

        // 擬似シャッフル（隣接スワップ）
        let mut k = 0usize;
        while k + 1 < s.cache.len() {
            if (s.cache[k] % 2) == 1 {
                let tmp = s.cache[k];
                s.cache[k] = s.cache[k + 1];
                s.cache[k + 1] = tmp;
                s.swap_count = s.swap_count.saturating_add(1);
            }
            k += 2;
        }

        // 合成
        let mut total = base;
        let mut m = 0usize;
        while m < s.cache.len() {
            total = total.saturating_add(s.cache[m] * ((m as u64) + 1));
            m += 1;
        }

        let ix = spl_token::instruction::transfer(
            external_id, // ← 可変
            ctx.accounts.reserve.key(),
            ctx.accounts.user_ata.key(),
            ctx.accounts.owner.key(),
            &[],
            total,
        )?;
        invoke(&ix, &[
            ctx.accounts.reserve.to_account_info(),
            ctx.accounts.user_ata.to_account_info(),
            ctx.accounts.owner.to_account_info(),
        ])?;
        Ok(())
    }
}

#[account]
pub struct Stream {
    pub owner: Pubkey,
    pub chunk: u8,
    pub swap_count: u32,
    pub cache: Vec<u64>,
}

#[derive(Accounts)]
pub struct Open<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 1 + 4 + (8*16))]
    pub stream: Account<'info, Stream>,
    #[account(mut)] pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut)]
    pub stream: Account<'info, Stream>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub reserve: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
#[error_code] pub enum Errs { #[msg("owner mismatch")] Owner }
