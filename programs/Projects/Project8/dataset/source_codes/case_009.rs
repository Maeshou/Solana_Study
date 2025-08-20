// Program 8: badge_minter (SPL Token: mint_to / burn は固定ID; PDA署名で権限行使)
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Mint, TokenAccount, MintTo, Burn};

declare_id!("BadgeM1nter8888888888888888888888888888");

#[program]
pub mod badge_minter {
    use super::*;

    pub fn init_badge(ctx: Context<InitBadge>, code: u64) -> Result<()> {
        let b = &mut ctx.accounts.badge;
        b.authority = ctx.accounts.issuer.key();
        b.code = code.rotate_left(2).wrapping_add(33);
        Ok(())
    }

    pub fn mint_to_user(ctx: Context<MintToUser>, amount: u64) -> Result<()> {
        let bump = *ctx.bumps.get("badge").ok_or(error!(E::MissingBump))?;
        let seeds: &[&[u8]] = &[b"badge", ctx.accounts.issuer.key.as_ref(), &ctx.accounts.badge.code.to_le_bytes(), &[bump]];

        let cpi_accounts = MintTo {
            mint: ctx.accounts.nft_mint.to_account_info(),
            to: ctx.accounts.user_token.to_account_info(),
            authority: ctx.accounts.badge.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts, &[seeds]);
        token::mint_to(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn burn_from_user(ctx: Context<BurnFromUser>, amount: u64) -> Result<()> {
        let bump = *ctx.bumps.get("badge").ok_or(error!(E::MissingBump))?;
        let seeds: &[&[u8]] = &[b"badge", ctx.accounts.issuer.key.as_ref(), &ctx.accounts.badge.code.to_le_bytes(), &[bump]];
        let cpi_accounts = Burn {
            mint: ctx.accounts.nft_mint.to_account_info(),
            from: ctx.accounts.user_token.to_account_info(),
            authority: ctx.accounts.badge.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts, &[seeds]);
        token::burn(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBadge<'info> {
    #[account(
        init,
        payer = issuer,
        space = 8 + 32 + 8,
        seeds=[b"badge", issuer.key().as_ref(), code.to_le_bytes().as_ref()],
        bump
    )]
    pub badge: Account<'info, Badge>,
    #[account(mut)]
    pub issuer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub code: u64,
}

#[derive(Accounts)]
pub struct MintToUser<'info> {
    #[account(
        mut,
        seeds=[b"badge", issuer.key().as_ref(), badge.code.to_le_bytes().as_ref()],
        bump
    )]
    pub badge: Account<'info, Badge>,
    pub nft_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub issuer: Signer<'info>,
}

#[derive(Accounts)]
pub struct BurnFromUser<'info> {
    #[account(
        mut,
        seeds=[b"badge", issuer.key().as_ref(), badge.code.to_le_bytes().as_ref()],
        bump
    )]
    pub badge: Account<'info, Badge>,
    pub nft_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub issuer: Signer<'info>,
}

#[account]
pub struct Badge { pub authority: Pubkey, pub code: u64 }

#[error_code]
pub enum E { #[msg("missing bump")] MissingBump }
