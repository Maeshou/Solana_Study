// =====================================
// 1. Token Transfer Program (安全なOwner Check)
// =====================================
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("11111111111111111111111111111111");

#[program]
pub mod secure_token_transfer {
    use super::*;

    pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        // Owner checkを実装 - token programが所有していることを確認
        require!(
            ctx.accounts.from.owner == &token::ID,
            ErrorCode::InvalidTokenAccountOwner
        );
        require!(
            ctx.accounts.to.owner == &token::ID,
            ErrorCode::InvalidTokenAccountOwner
        );

        let transfer_instruction = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
        );

        token::transfer(cpi_ctx, amount)
    }
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut, constraint = from.owner == &token::ID)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut, constraint = to.owner == &token::ID)]
    pub to: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid token account owner")]
    InvalidTokenAccountOwner,
}