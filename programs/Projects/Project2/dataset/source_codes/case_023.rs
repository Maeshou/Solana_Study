// =====================================
// 3. Vault Program (資金管理)
// =====================================
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("33333333333333333333333333333333");

#[program]
pub mod secure_vault {
    use super::*;

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        // 複数のowner checkを実装
        require!(
            ctx.accounts.user_token_account.owner == &token::ID,
            ErrorCode::InvalidUserTokenOwner
        );
        require!(
            ctx.accounts.vault_token_account.owner == &token::ID,
            ErrorCode::InvalidVaultTokenOwner
        );
        require!(
            ctx.accounts.vault.to_account_info().owner == ctx.program_id,
            ErrorCode::InvalidVaultOwner
        );

        let vault = &mut ctx.accounts.vault;
        vault.total_deposited += amount;

        let transfer_instruction = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
        );

        token::transfer(cpi_ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        // 厳密なowner checkを実装
        let vault_info = ctx.accounts.vault.to_account_info();
        require!(
            vault_info.owner == ctx.program_id,
            ErrorCode::InvalidVaultOwner
        );

        let vault = &mut ctx.accounts.vault;
        require!(
            vault.total_deposited >= amount,
            ErrorCode::InsufficientFunds
        );

        vault.total_deposited -= amount;

        let seeds = &[
            b"vault",
            ctx.accounts.vault.authority.as_ref(),
            &[ctx.accounts.vault.bump],
        ];
        let signer = &[&seeds[..]];

        let transfer_instruction = Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
            signer,
        );

        token::transfer(cpi_ctx, amount)
    }
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        mut,
        constraint = vault.to_account_info().owner == program_id
    )]
    pub vault: Account<'info, Vault>,
    #[account(
        mut,
        constraint = user_token_account.owner == &token::ID
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = vault_token_account.owner == &token::ID
    )]
    pub vault_token_account: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        has_one = authority,
        constraint = vault.to_account_info().owner == program_id
    )]
    pub vault: Account<'info, Vault>,
    #[account(
        mut,
        constraint = user_token_account.owner == &token::ID
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = vault_token_account.owner == &token::ID
    )]
    pub vault_token_account: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub total_deposited: u64,
    pub bump: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid user token account owner")]
    InvalidUserTokenOwner,
    #[msg("Invalid vault token account owner")]
    InvalidVaultTokenOwner,
    #[msg("Invalid vault account owner")]
    InvalidVaultOwner,
    #[msg("Insufficient funds")]
    InsufficientFunds,
}