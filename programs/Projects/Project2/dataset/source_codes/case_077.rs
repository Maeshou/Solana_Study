use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, Burn};

declare_id!("TokenDispt77777777777777777777777777777777");

#[program]
pub mod token_dispatch {
    use super::*;

    pub fn send(ctx: Context<Send>, amount: u64) -> Result<()> {
        require!(ctx.accounts.from.amount >= amount, ErrorCode::InsufficientFunds);
        let cpi = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.from.to_account_info(),
                to: ctx.accounts.to.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        anchor_spl::token::transfer(cpi, amount)?;
        emit!(TokensSent {
            from: ctx.accounts.user.key(),
            to: ctx.accounts.to.key(),
            amount
        });
        Ok(())
    }

    pub fn burn_tokens(ctx: Context<BurnTokens>, amount: u64) -> Result<()> {
        let cpi = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.from.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        anchor_spl::token::burn(cpi, amount)?;
        emit!(TokensBurned {
            burner: ctx.accounts.user.key(),
            amount
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Send<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct BurnTokens<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    pub mint: Account<'info, Token>,
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[event]
pub struct TokensSent {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
}

#[event]
pub struct TokensBurned {
    pub burner: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Not enough funds")]
    InsufficientFunds,
}
