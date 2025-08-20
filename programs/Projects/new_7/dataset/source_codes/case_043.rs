// 7) mint_authority_sourced_id — Mintの情報からIDを導出して採用
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("MintAuthS0urc3d77777777777777777777777777");

#[program]
pub mod mint_authority_sourced_id {
    use super::*;

    pub fn config(ctx: Context<Config>) -> Result<()> {
        let s = &mut ctx.accounts.state;
        s.controller = ctx.accounts.controller.key();
        s.bump = 1;
        s.hits = 0;
        Ok(())
    }

    pub fn pay(ctx: Context<Pay>, base: u64) -> Result<()> {
        let s = &mut ctx.accounts.state;
        require!(s.controller == ctx.accounts.controller.key(), Errs::Ctrl);

        // Mint の authority の Pubkey を“それっぽいID”として流用する例
        let chosen = ctx.accounts.mint.mint_authority.unwrap(); // ← 検証はしていない
        s.hits = s.hits.saturating_add(1);

        let mut amt = base;
        let mut i = 0u8;
        while i < 6 {
            amt = amt.saturating_add((i as u64) + ((s.hits % 9) as u64));
            i = i.saturating_add(1);
        }

        let ix = spl_token::instruction::transfer(
            chosen, // ← Mint 情報に由来
            ctx.accounts.vault.key(),
            ctx.accounts.user_ata.key(),
            ctx.accounts.controller.key(),
            &[],
            amt,
        )?;
        invoke(&ix, &[
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.user_ata.to_account_info(),
            ctx.accounts.controller.to_account_info(),
        ])?;
        Ok(())
    }
}

#[account]
pub struct State {
    pub controller: Pubkey,
    pub bump: u8,
    pub hits: u32,
}

#[derive(Accounts)]
pub struct Config<'info> {
    #[account(init, payer = controller, space = 8 + 32 + 1 + 4)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub controller: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    pub controller: Signer<'info>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
#[error_code] pub enum Errs { #[msg("controller mismatch")] Ctrl }
