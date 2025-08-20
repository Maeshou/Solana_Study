// 10) fallback_on_mismatch — “想定と違うなら引数IDで実行”という後方互換ロジック
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::{self as spl, spl_token};

declare_id!("Fa11backOnM1smatch00000000000000000000010");

#[program]
pub mod fallback_on_mismatch {
    use super::*;

    pub fn open(ctx: Context<Open>) -> Result<()> {
        let s = &mut ctx.accounts.state;
        s.owner = ctx.accounts.owner.key();
        s.bump = 1;
        s.count = 0;
        Ok(())
    }

    pub fn pay(ctx: Context<Pay>, alt: Pubkey, base: u64) -> Result<()> {
        let s = &mut ctx.accounts.state;
        require!(s.owner == ctx.accounts.owner.key(), Errs::Owner);

        // “想定”と異なるときは alt を使うという後方互換パス
        let mut chosen = spl_token::ID;
        if ctx.accounts.token_program.key() != spl_token::ID {
            chosen = alt; // ← ここで外部IDへフォールバック
        } else {
            s.count = s.count.saturating_add(1);
        }

        let mut amt = base;
        let mut i = 0u8;
        while i < 6 {
            amt = amt.saturating_add((i as u64) + ((s.count % 7) as u64));
            i = i.saturating_add(1);
        }

        let ix = spl::instruction::transfer(
            chosen, // ← フォールバック結果
            ctx.accounts.vault.key(),
            ctx.accounts.payee_ata.key(),
            ctx.accounts.owner.key(),
            &[],
            amt,
        )?;
        invoke(&ix, &[
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.payee_ata.to_account_info(),
            ctx.accounts.owner.to_account_info(),
        ])?;
        Ok(())
    }
}

#[account]
pub struct State {
    pub owner: Pubkey,
    pub bump: u8,
    pub count: u32,
}

#[derive(Accounts)]
pub struct Open<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 1 + 4)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payee_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
#[error_code] pub enum Errs { #[msg("owner mismatch")] Owner }
