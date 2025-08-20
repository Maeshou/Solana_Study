use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};

declare_id!("Escrow11111111111111111111111111111111111");

#[program]
pub mod token_escrow {
    use super::*;
    pub fn lock_tokens(ctx: Context<LockTokens>, amount: u64) -> Result<()> {
        let cpi_accounts = token::Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LockTokens<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    // エスクロー設定アカウント。このプログラムが所有者であることを検証。
    #[account(
        seeds = [b"escrow_config"],
        bump
    )]
    pub escrow_config: Account<'info, EscrowConfig>,

    // Vault (PDA) が所有するトークンアカウント。
    #[account(
        mut,
        constraint = vault.owner == escrow_config.key()
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[account]
pub struct EscrowConfig {
    pub admin: Pubkey,
}