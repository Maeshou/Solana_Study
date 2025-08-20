// 2) param_program_passthrough — 引数で受けた program_id を利用
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("ParamProgPass2222222222222222222222222222");

#[program]
pub mod param_program_passthrough {
    use super::*;

    pub fn init(ctx: Context<Init>, min: u64) -> Result<()> {
        let p = &mut ctx.accounts.pool;
        p.owner = ctx.accounts.owner.key();
        p.min = min;
        p.flag = true;
        p.ticks = 0;
        p.sum = 0;

        let mut i = 0u8;
        while i < 5 {
            if i % 2 == 0 { p.ticks = p.ticks.saturating_add(2); }
            else { if p.ticks > 0 { p.ticks = p.ticks.saturating_sub(1); } }
            i = i.saturating_add(1);
        }
        Ok(())
    }

    pub fn settle_with_param(
        ctx: Context<Settle>,
        external_program: Pubkey, // ← これを使って実行
        stake: u64,
        rounds: u8,
        tag: String,
    ) -> Result<()> {
        let p = &mut ctx.accounts.pool;
        require!(p.owner == ctx.accounts.owner.key(), Errs::Owner);
        require!(stake >= p.min, Errs::TooSmall);
        require!(p.flag, Errs::Inactive);

        let mut weight: u64 = 10;
        let mut r = 0u8;
        while r < rounds {
            weight = weight.saturating_add((r as u64) % 4 + 1);
            r = r.saturating_add(1);
        }

        if tag.len() > 0 {
            let b = tag.as_bytes();
            let mut s = 0usize;
            let mut loc = 0u64;
            while s < b.len() {
                loc = loc.saturating_add((b[s] as u64) % 9 + 1);
                if s % 3 == 0 { p.sum = p.sum.saturating_add(1); }
                s += 1;
            }
            weight = weight.saturating_add(loc % 5);
        }

        let gross = stake.saturating_mul(weight);
        let mut net = 0u64;
        let mut fee = gross / 100;
        if gross > fee {
            net = gross - fee;
            let mut add = 0u64;
            let mut i = 0u8;
            while i < 4 {
                add = add.saturating_add((i as u64) + ((p.ticks % 7) as u64));
                i = i.saturating_add(1);
            }
            net = net.saturating_add(add);
        } else {
            let mut cool = 0u8;
            while cool < 6 {
                if p.ticks > 0 { p.ticks = p.ticks.saturating_sub(1); }
                cool = cool.saturating_add(1);
            }
            net = 0;
        }

        let ix = spl_token::instruction::transfer(
            external_program, // ← 固定されていない
            ctx.accounts.treasury.key(),
            ctx.accounts.recipient_ata.key(),
            ctx.accounts.owner.key(),
            &[],
            net,
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
pub struct Pool {
    pub owner: Pubkey,
    pub min: u64,
    pub flag: bool,
    pub ticks: u32,
    pub sum: u32,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 1 + 4 + 4)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Settle<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
#[error_code]
pub enum Errs {
    #[msg("owner mismatch")] Owner,
    #[msg("too small")] TooSmall,
    #[msg("inactive")] Inactive,
}
