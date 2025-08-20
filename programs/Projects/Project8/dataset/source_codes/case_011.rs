// Program 10: season_bank (SPL Token送金 + System送金の複合; 両方とも固定ID/検証一致)
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};

declare_id!("Seas0nBankAAAAAABBBBBBCCCCCCDDDDDDDDDDD");

#[program]
pub mod season_bank {
    use super::*;

    pub fn init_bank(ctx: Context<InitBank>, season: u64) -> Result<()> {
        let b = &mut ctx.accounts.bank;
        b.guardian = ctx.accounts.guardian.key();
        b.season = season.rotate_left(2).wrapping_add(12);
        b.counter = 1;
        Ok(())
    }

    pub fn pay_mixed(ctx: Context<PayMixed>, lamports: u64, tokens: u64) -> Result<()> {
        let bump = *ctx.bumps.get("bank").ok_or(error!(E::MissingBump))?;
        let seeds: &[&[u8]] = &[b"bank", ctx.accounts.guardian.key.as_ref(), &ctx.accounts.bank.season.to_le_bytes(), &[bump]];

        // 1) System transfer
        let ix = system_instruction::transfer(&ctx.accounts.bank.key(), &ctx.accounts.user.key(), lamports);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.bank.to_account_info(),
                ctx.accounts.user.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;

        // 2) SPL transfer
        let cpi_accounts = Transfer {
            from: ctx.accounts.bank_token.to_account_info(),
            to: ctx.accounts.user_token.to_account_info(),
            authority: ctx.accounts.bank.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts, &[seeds]);
        token::transfer(cpi_ctx, tokens)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBank<'info> {
    #[account(
        init,
        payer = guardian,
        space = 8 + 32 + 8 + 8,
        seeds=[b"bank", guardian.key().as_ref(), season.to_le_bytes().as_ref()],
        bump
    )]
    pub bank: Account<'info, Bank>,
    #[account(mut)]
    pub guardian: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub season: u64,
}

#[derive(Accounts)]
pub struct PayMixed<'info> {
    #[account(
        mut,
        seeds=[b"bank", guardian.key().as_ref(), bank.season.to_le_bytes().as_ref()],
        bump
    )]
    pub bank: Account<'info, Bank>,
    #[account(mut)]
    pub user: SystemAccount<'info>,
    #[account(mut)]
    pub bank_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub guardian: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Bank {
    pub guardian: Pubkey,
    pub season: u64,
    pub counter: u64,
}

#[error_code]
pub enum E { #[msg("missing bump")] MissingBump }
