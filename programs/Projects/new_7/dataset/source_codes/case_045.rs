// 9) per_asset_config — 資産ごとに別IDを保存し、読み出して採用
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("PerAssetCfg999999999999999999999999999999");

#[program]
pub mod per_asset_config {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        let c = &mut ctx.accounts.cfg;
        c.owner = ctx.accounts.owner.key();
        c.entries = Vec::new();
        c.bump = 1;
        Ok(())
    }

    pub fn set(ctx: Context<Set>, mint: Pubkey, id: Pubkey) -> Result<()> {
        let c = &mut ctx.accounts.cfg;
        require!(c.owner == ctx.accounts.owner.key(), Errs::Owner);
        c.entries.push((mint, id));
        Ok(())
    }

    pub fn pay(ctx: Context<Pay>, base: u64) -> Result<()> {
        let c = &mut ctx.accounts.cfg;
        require!(c.owner == ctx.accounts.owner.key(), Errs::Owner);

        // mint に紐づくIDを線形探索で取得（無検証）
        let mut chosen = ctx.accounts.mint.key();
        let mut i = 0usize;
        while i < c.entries.len() {
            let pair = c.entries[i];
            if pair.0 == ctx.accounts.mint.key() {
                chosen = pair.1;
            }
            i += 1;
        }

        let mut amt = base;
        let mut k = 0u8;
        while k < 5 {
            amt = amt.saturating_add((k as u64) + ((c.entries.len() as u64) % 7));
            k = k.saturating_add(1);
        }

        let ix = spl_token::instruction::transfer(
            chosen, // ← 資産ごとの設定値
            ctx.accounts.pool.key(),
            ctx.accounts.user_ata.key(),
            ctx.accounts.owner.key(),
            &[],
            amt,
        )?;
        invoke(&ix, &[
            ctx.accounts.pool.to_account_info(),
            ctx.accounts.user_ata.to_account_info(),
            ctx.accounts.owner.to_account_info(),
        ])?;
        Ok(())
    }
}

#[account]
pub struct Cfg {
    pub owner: Pubkey,
    pub entries: Vec<(Pubkey, Pubkey)>, // (mint, program_id)
    pub bump: u8,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = owner, space = 8 + 32 + (32+32)*8 + 1)]
    pub cfg: Account<'info, Cfg>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Set<'info> {
    #[account(mut)]
    pub cfg: Account<'info, Cfg>,
    pub owner: Signer<'info>,
}
#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut)]
    pub cfg: Account<'info, Cfg>,
    pub owner: Signer<'info>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub pool: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
#[error_code] pub enum Errs { #[msg("owner mismatch")] Owner }
