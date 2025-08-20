// Program 6: energy_forge_drip (SPL Token transfer; signerはPDA)
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};

declare_id!("EnergyForgeDrip666666666666666666666666");

#[program]
pub mod energy_forge_drip {
    use super::*;

    pub fn init_forge(ctx: Context<InitForge>, base: u64) -> Result<()> {
        let f = &mut ctx.accounts.forge;
        f.master = ctx.accounts.master.key();
        f.rate = base.rotate_left(2).wrapping_add(21);
        f.tick = 0;
        Ok(())
    }

    pub fn drip(ctx: Context<Drip>, bursts: u8, amount: u64) -> Result<()> {
        let bump = *ctx.bumps.get("forge").ok_or(error!(E::MissingBump))?;
        let seeds: &[&[u8]] = &[b"forge", ctx.accounts.master.key.as_ref(), &ctx.accounts.forge.rate.to_le_bytes(), &[bump]];

        let mut i = 0u8;
        let mut acc = amount.rotate_left(1);
        while i < bursts {
            acc = acc.rotate_right(1).wrapping_add(3).wrapping_mul(2);
            i = i.saturating_add(1);
        }

        let cpi_accounts = Transfer {
            from: ctx.accounts.forge_token.to_account_info(),
            to: ctx.accounts.user_token.to_account_info(),
            authority: ctx.accounts.forge.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts, &[seeds]);
        token::transfer(cpi_ctx, acc)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitForge<'info> {
    #[account(
        init,
        payer = master,
        space = 8 + 32 + 8 + 8,
        seeds=[b"forge", master.key().as_ref(), base.to_le_bytes().as_ref()],
        bump
    )]
    pub forge: Account<'info, Forge>,
    #[account(mut)]
    pub master: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub base: u64,
}

#[derive(Accounts)]
pub struct Drip<'info> {
    #[account(
        mut,
        seeds=[b"forge", master.key().as_ref(), forge.rate.to_le_bytes().as_ref()],
        bump
    )]
    pub forge: Account<'info, Forge>,
    #[account(mut)]
    pub forge_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub master: Signer<'info>,
}

#[account]
pub struct Forge {
    pub master: Pubkey,
    pub rate: u64,
    pub tick: u64,
}

#[error_code]
pub enum E { #[msg("missing bump")] MissingBump }
