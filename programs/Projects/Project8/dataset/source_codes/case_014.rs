// Program 3: music_royalty_pool （音楽ロイヤリティ分配）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("Mus1cRoyaltyP00l33333333333333333333333");

#[program]
pub mod music_royalty_pool {
    use super::*;

    pub fn init_pool(ctx: Context<InitPool>, era: u64) -> Result<()> {
        let s = &mut ctx.accounts.stage;
        s.curator = ctx.accounts.curator.key();
        s.era = era.rotate_left(3).wrapping_add(17);
        s.pitch = s.era.rotate_right(1).wrapping_add(11);
        Ok(())
    }

    pub fn stream_and_tip(ctx: Context<StreamAndTip>, tokens: u64, lamports: u64) -> Result<()> {
        let bump = *ctx.bumps.get("stage").ok_or(error!(E::MissingBump))?;
        let seeds: &[&[u8]] = &[b"stage", ctx.accounts.curator.key.as_ref(), &ctx.accounts.stage.era.to_le_bytes(), &[bump]];

        // SPLトークンで配布
        let cpi_accounts = Transfer {
            from: ctx.accounts.stage_token.to_account_info(),
            to: ctx.accounts.artist_token.to_account_info(),
            authority: ctx.accounts.stage.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts, &[seeds]);

        let mut delivered = 0u64;
        let mut step = (tokens / 4).max(1);
        while delivered < tokens {
            let send = (tokens - delivered).min(step);
            token::transfer(cpi_ctx.clone(), send)?;
            delivered = delivered.saturating_add(send);
            step = step.rotate_left(1).wrapping_add(1);
            if step > (tokens - delivered) && (tokens - delivered) > 5 { step = (tokens - delivered) - 2; }
        }

        // 追加チップ（System）
        let ix = system_instruction::transfer(&ctx.accounts.stage.key(), &ctx.accounts.artist_wallet.key(), lamports);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.stage.to_account_info(),
                ctx.accounts.artist_wallet.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPool<'info> {
    #[account(
        init,
        payer = curator,
        space = 8 + 32 + 8 + 8,
        seeds=[b"stage", curator.key().as_ref(), era.to_le_bytes().as_ref()],
        bump
    )]
    pub stage: Account<'info, Stage>,
    #[account(mut)]
    pub curator: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub era: u64,
}

#[derive(Accounts)]
pub struct StreamAndTip<'info> {
    #[account(
        mut,
        seeds=[b"stage", curator.key().as_ref(), stage.era.to_le_bytes().as_ref()],
        bump
    )]
    pub stage: Account<'info, Stage>,
    #[account(mut)]
    pub stage_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub artist_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub artist_wallet: SystemAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub curator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Stage {
    pub curator: Pubkey,
    pub era: u64,
    pub pitch: u64,
}

#[error_code]
pub enum E { #[msg("missing bump")] MissingBump }
