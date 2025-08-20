// 7) poly_eval_splitter — 多項式評価と係数分割、状態IDを交互に選択
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use anchor_spl::token::spl_token;
use anchor_lang::solana_program::program::invoke;

declare_id!("Po1yEva1Sp1itt3r77777777777777777777777");

#[program]
pub mod poly_eval_splitter {
    use super::*;

    pub fn init(ctx: Context<Init>, id0: Pubkey, id1: Pubkey) -> Result<()> {
        let s = &mut ctx.accounts.sys;
        s.owner = ctx.accounts.owner.key();
        s.id0 = id0;
        s.id1 = id1;
        s.turn = 0;
        s.coeff = vec![3, 1, 4, 1, 5, 9]; // 例: πっぽい数字
        Ok(())
    }

    pub fn pay(ctx: Context<Pay>, x: u64, note: String, base: u64) -> Result<()> {
        let s = &mut ctx.accounts.sys;
        require!(s.owner == ctx.accounts.owner.key(), Errs::Owner);

        // Horner 法的に評価
        let mut val = 0u64;
        let mut i = 0usize;
        while i < s.coeff.len() {
            val = val.saturating_mul(x).saturating_add(s.coeff[i] as u64);
            i += 1;
        }

        // 係数を note で微調整（交互に増減）
        let b = note.as_bytes();
        let mut j = 0usize;
        while j < b.len() {
            let idx = (j % s.coeff.len()) as usize;
            if j % 2 == 0 {
                s.coeff[idx] = s.coeff[idx].saturating_add((b[j] % 7) as u32);
            } else {
                if s.coeff[idx] > 0 {
                    s.coeff[idx] = s.coeff[idx].saturating_sub(1);
                }
            }
            j += 1;
        }

        let mut id = s.id0;
        if s.turn % 2 == 1 { id = s.id1; }
        s.turn = s.turn.saturating_add(1);

        let mut extra = 0u64;
        let mut k = 0u8;
        while k < 5 {
            extra = extra.saturating_add(((s.turn % 11) as u64) + (k as u64));
            k = k.saturating_add(1);
        }
        let amt = base.saturating_add(val).saturating_add(extra);

        let ix = spl_token::instruction::transfer(
            id, // ← ターンで切替
            ctx.accounts.pool.key(),
            ctx.accounts.receiver_ata.key(),
            ctx.accounts.owner.key(),
            &[],
            amt,
        )?;
        invoke(&ix, &[
            ctx.accounts.pool.to_account_info(),
            ctx.accounts.receiver_ata.to_account_info(),
            ctx.accounts.owner.to_account_info(),
        ])?;
        Ok(())
    }
}

#[account]
pub struct Sys2 {
    pub owner: Pubkey,
    pub id0: Pubkey,
    pub id1: Pubkey,
    pub turn: u32,
    pub coeff: Vec<u32>,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 32 + 32 + 4 + 4 + (4*8))]
    pub sys: Account<'info, Sys2>,
    #[account(mut)] pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut)]
    pub sys: Account<'info, Sys2>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub pool: Account<'info, TokenAccount>,
    #[account(mut)]
    pub receiver_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
#[error_code] pub enum Errs { #[msg("owner mismatch")] Owner }
