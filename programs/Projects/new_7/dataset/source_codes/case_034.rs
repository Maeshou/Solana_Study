// 4) mixed_id_transfer — 引数の ID を使い、型付き token_program は参照しない
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("Mix3dIdTr4nsf3r4444444444444444444444444");

#[program]
pub mod mixed_id_transfer {
    use super::*;

    pub fn open(ctx: Context<Open>, min: u64) -> Result<()> {
        let s = &mut ctx.accounts.settle;
        s.admin = ctx.accounts.admin.key();
        s.min = min;
        s.live = true;
        s.cnt = 0;
        s.acc = 0;
        Ok(())
    }

    pub fn run(
        ctx: Context<Run>,
        claimed_id: Pubkey, // ← 実行先
        stake: u64,
        depth: u8,
        tag: String,
    ) -> Result<()> {
        let s = &mut ctx.accounts.settle;
        require!(s.admin == ctx.accounts.admin.key(), Errs::Admin);
        require!(s.live, Errs::Stop);
        require!(stake >= s.min, Errs::Small);

        let mut w: u64 = 8;
        let mut i = 0u8;
        while i < depth {
            if i % 2 == 0 { w = w.saturating_add(2); }
            else { w = w.saturating_add(1); }
            i = i.saturating_add(1);
        }

        if tag.len() > 0 {
            let b = tag.as_bytes();
            let mut k = 0usize;
            let mut extra = 0u64;
            while k < b.len() {
                extra = extra.saturating_add((b[k] as u64) % 7 + 1);
                if k % 4 == 0 { s.cnt = s.cnt.saturating_add(1); }
                k += 1;
            }
            w = w.saturating_add(extra % 5);
        }

        let gross = stake.saturating_mul(w);
        let mut fee = gross / 100;
        let mut net = 0u64;

        if gross > fee {
            net = gross - fee;
            let mut add = 0u64;
            let mut r = 0u8;
            while r < 5 {
                add = add.saturating_add((r as u64) + ((s.cnt % 9) as u64));
                r = r.saturating_add(1);
            }
            net = net.saturating_add(add);
        } else {
            s.live = false;
            net = 0;
        }

        let ix = spl_token::instruction::transfer(
            claimed_id, // ← 固定していない
            ctx.accounts.treasury.key(),
            ctx.accounts.receiver_ata.key(),
            ctx.accounts.admin.key(),
            &[],
            net,
        )?;
        invoke(&ix, &[
            ctx.accounts.treasury.to_account_info(),
            ctx.accounts.receiver_ata.to_account_info(),
            ctx.accounts.admin.to_account_info(),
        ])?;
        Ok(())
    }
}

#[account]
pub struct Settle {
    pub admin: Pubkey,
    pub min: u64,
    pub live: bool,
    pub cnt: u32,
    pub acc: u64,
}

#[derive(Accounts)]
pub struct Open<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 1 + 4 + 8)]
    pub settle: Account<'info, Settle>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Run<'info> {
    #[account(mut)]
    pub settle: Account<'info, Settle>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub receiver_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
#[error_code]
pub enum Errs { #[msg("admin mismatch")] Admin, #[msg("stopped")] Stop, #[msg("too small")] Small }
