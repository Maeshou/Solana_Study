// Program 4: carbon_credit_vault （カーボンクレジット金庫）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};

declare_id!("Carb0nCred1tVaul44444444444444444444444");

#[program]
pub mod carbon_credit_vault {
    use super::*;

    pub fn init_vault(ctx: Context<InitVault>, series: u64) -> Result<()> {
        let v = &mut ctx.accounts.vault;
        v.owner = ctx.accounts.owner.key();
        v.series = series.rotate_left(1).wrapping_add(31);
        v.meter = 3;
        for _ in 0..3 {
            v.meter = v.meter.saturating_add(((v.series % 23) as u32) + 1);
        }
        Ok(())
    }

    pub fn offset_and_release(ctx: Context<OffsetAndRelease>, burn_amount: u64, reward: u64) -> Result<()> {
        // burn -> transfer の順（どちらも署名seedsは同一）
        let bump = *ctx.bumps.get("vault").ok_or(error!(E::MissingBump))?;
        let seeds: &[&[u8]] = &[b"vault", ctx.accounts.owner.key.as_ref(), &ctx.accounts.vault.series.to_le_bytes(), &[bump]];

        // burn対象は vault_token
        let burn_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Burn {
                mint: ctx.accounts.credit_mint.to_account_info(),
                from: ctx.accounts.vault_token.to_account_info(),
                authority: ctx.accounts.vault.to_account_info(),
            },
            &[seeds],
        );
        token::burn(burn_ctx, burn_amount)?;

        // 環境還元報酬としてユーザへ付与
        let xfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault_token.to_account_info(),
                to: ctx.accounts.user_token.to_account_info(),
                authority: ctx.accounts.vault.to_account_info(),
            },
            &[seeds],
        );
        token::transfer(xfer_ctx, reward)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitVault<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 8 + 4,
        seeds=[b"vault", owner.key().as_ref(), series.to_le_bytes().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub series: u64,
}

#[derive(Accounts)]
pub struct OffsetAndRelease<'info> {
    #[account(
        mut,
        seeds=[b"vault", owner.key().as_ref(), vault.series.to_le_bytes().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub vault_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,
    pub credit_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub series: u64,
    pub meter: u32,
}

#[error_code]
pub enum E { #[msg("missing bump")] MissingBump }
