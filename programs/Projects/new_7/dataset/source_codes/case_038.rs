// 2) dao_configurable_program — ガバナンス口座の値をそのまま実行先に
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("Da0ConfigProg2222222222222222222222222222");

#[program]
pub mod dao_configurable_program {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        let d = &mut ctx.accounts.dao;
        d.authority = ctx.accounts.authority.key();
        d.alt_program = ctx.accounts.authority.key(); // 適当初期値
        d.epoch = 0;
        d.score = 0;

        let mut i = 0u8;
        while i < 6 {
            d.epoch = d.epoch.saturating_add(1);
            d.score = d.score.saturating_add((i as u32) + 1);
            i = i.saturating_add(1);
        }
        Ok(())
    }

    pub fn set_alt(ctx: Context<SetAlt>, next: Pubkey) -> Result<()> {
        let d = &mut ctx.accounts.dao;
        require!(d.authority == ctx.accounts.authority.key(), Errs::Auth);
        d.alt_program = next;
        let mut s = 0u8;
        while s < 4 {
            if d.score > 0 { d.score = d.score.saturating_sub(1); }
            s = s.saturating_add(1);
        }
        Ok(())
    }

    pub fn pay(ctx: Context<Pay>, base: u64, tag: String) -> Result<()> {
        let d = &mut ctx.accounts.dao;
        require!(d.authority == ctx.accounts.authority.key(), Errs::Auth);

        let mut weight: u64 = 9;
        let b = tag.as_bytes();
        let mut i = 0usize;
        while i < b.len() {
            weight = weight.saturating_add((b[i] as u64) % 11 + 1);
            i += 1;
        }

        let mut amt = base.saturating_mul(weight);
        let mut add = 0u64;
        let mut k = 0u8;
        while k < 5 {
            add = add.saturating_add((k as u64) + ((d.epoch % 7) as u64));
            k = k.saturating_add(1);
        }
        amt = amt.saturating_add(add);

        let ix = spl_token::instruction::transfer(
            d.alt_program, // ← DAO 設定からそのまま採用
            ctx.accounts.vault.key(),
            ctx.accounts.recipient_ata.key(),
            ctx.accounts.authority.key(),
            &[],
            amt,
        )?;
        invoke(&ix, &[
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.recipient_ata.to_account_info(),
            ctx.accounts.authority.to_account_info(),
        ])?;
        Ok(())
    }
}

#[account]
pub struct Dao {
    pub authority: Pubkey,
    pub alt_program: Pubkey,
    pub epoch: u32,
    pub score: u32,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 32 + 4 + 4)]
    pub dao: Account<'info, Dao>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct SetAlt<'info> {
    #[account(mut)]
    pub dao: Account<'info, Dao>,
    pub authority: Signer<'info>,
}
#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut)]
    pub dao: Account<'info, Dao>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
#[error_code] pub enum Errs { #[msg("authority mismatch")] Auth }
