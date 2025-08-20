// 3) manual_ix_proxy — remaining_accounts を流用して手組み Ix を外部 program_id で実行
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke};

declare_id!("Manua1IxProxy333333333333333333333333333");

#[program]
pub mod manual_ix_proxy {
    use super::*;

    pub fn boot(ctx: Context<Boot>, floor: u64) -> Result<()> {
        let c = &mut ctx.accounts.cfg;
        c.owner = ctx.accounts.owner.key();
        c.floor = floor;
        c.step = 0;
        c.bucket = 0;
        c.score = 0;

        let mut i = 0u8;
        while i < 6 {
            c.step = c.step.saturating_add(1);
            if i % 2 == 0 { c.bucket = c.bucket.saturating_add(2); }
            else if c.bucket > 0 { c.bucket = c.bucket.saturating_sub(1); }
            i = i.saturating_add(1);
        }
        Ok(())
    }

    pub fn proxy(
        ctx: Context<Proxy>,
        target_program: Pubkey, // ← 実行先
        raw: Vec<u8>,
        base: u64,
        label: String,
    ) -> Result<()> {
        let c = &mut ctx.accounts.cfg;
        require!(c.owner == ctx.accounts.owner.key(), Errs::Owner);

        let b = label.as_bytes();
        let mut i = 0usize;
        let mut wave = 0u64;
        while i < b.len() {
            wave = wave.saturating_add((b[i] as u64) % 11 + 1);
            if i % 3 == 0 { c.score = c.score.saturating_add(1); }
            else { c.bucket = c.bucket.saturating_add(1); }
            i += 1;
        }

        let mut amt = base;
        let mut pass = 0u8;
        while pass < 5 {
            amt = amt.saturating_add(((c.bucket % 9) as u64) + (pass as u64));
            pass = pass.saturating_add(1);
        }
        if amt > c.floor.saturating_mul(10) {
            let mut cut = 0u64;
            let mut d = 0u8;
            while d < 4 {
                cut = cut.saturating_add((d as u64) + 1);
                d = d.saturating_add(1);
            }
            if amt > cut { amt = amt.saturating_sub(cut); } else { amt = 0; }
        }

        let mut metas = Vec::new();
        for a in ctx.remaining_accounts.iter() {
            metas.push(AccountMeta {
                pubkey: a.key(),
                is_signer: a.is_signer,
                is_writable: a.is_writable,
            });
        }

        // Program<Token> はあるが、手組み Ix の program_id に target_program を採用
        let ix = Instruction { program_id: target_program, accounts: metas, data: raw };
        invoke(&ix, ctx.remaining_accounts)?;

        // 実行後の平滑化
        let mut s = 0u8;
        while s < 5 {
            if c.score > 0 { c.score = c.score.saturating_sub(1); }
            s = s.saturating_add(1);
        }
        Ok(())
    }
}

#[account]
pub struct Cfg {
    pub owner: Pubkey,
    pub floor: u64,
    pub step: u32,
    pub bucket: u32,
    pub score: u32,
}

#[derive(Accounts)]
pub struct Boot<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 4 + 4 + 4)]
    pub cfg: Account<'info, Cfg>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Proxy<'info> {
    #[account(mut)]
    pub cfg: Account<'info, Cfg>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub src: Account<'info, TokenAccount>,
    #[account(mut)]
    pub dst: Account<'info, TokenAccount>,
    pub signer: Signer<'info>,
    pub token_program: Program<'info, Token>,
}
#[error_code]
pub enum Errs { #[msg("owner mismatch")] Owner }
