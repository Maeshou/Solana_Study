use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Token, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWxqSWX6oWlekghXz3FvU8aXYZ12"); // replace with your program ID

#[program]
pub mod nft_reward_withdraw {
    use super::*;

    /// Initialize the vault: set the issuer and the reward-to-token rate.
    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
        issuer: Pubkey,
        tokens_per_reward: u64,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.issuer = issuer;
        vault.tokens_per_reward = tokens_per_reward;
        Ok(())
    }

    /// Withdraw tokens based on the reward amount supplied by the NFT issuer.
    /// ⚠️ Missing signer check on `issuer` – vulnerable!
    pub fn withdraw_tokens(
        ctx: Context<WithdrawTokens>,
        reward_amount: u64,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let token_vault = &ctx.accounts.token_vault;
        let destination = &ctx.accounts.destination;

        // Intentionally missing:
        // assert!(ctx.accounts.issuer.is_signer, "Issuer must sign this transaction");

        // Calculate how many tokens to withdraw
        let to_withdraw = vault
            .tokens_per_reward
            .checked_mul(reward_amount)
            .ok_or(ErrorCode::MathOverflow)?;

        // Perform SPL token transfer CPI
        let cpi_accounts = Transfer {
            from: token_vault.to_account_info(),
            to: destination.to_account_info(),
            authority: ctx.accounts.token_vault_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        token::transfer(
            CpiContext::new(cpi_program, cpi_accounts),
            to_withdraw,
        )?;

        Ok(())
    }
}

/// Vault state: tracks issuer and the rate of tokens per reward unit.
#[account]
pub struct Vault {
    /// The NFT issuer who may (but is not checked to) sign withdrawals.
    pub issuer: Pubkey,
    /// How many tokens to withdraw per 1 reward unit.
    pub tokens_per_reward: u64,
}

/// Accounts for initialization
#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 8)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Accounts for withdrawing tokens
#[derive(Accounts)]
pub struct WithdrawTokens<'info> {
    #[account(mut, has_one = issuer)]
    pub vault: Account<'info, Vault>,
    /// CHECK: intentionally unchecked to illustrate missing signer check vulnerability
    pub issuer: UncheckedAccount<'info>,
    #[account(mut)]
    pub token_vault: Account<'info, TokenAccount>,
    /// Authority over the token vault (e.g., PDA); assumed correct but not verified against issuer
    pub token_vault_authority: Signer<'info>,
    #[account(mut)]
    pub destination: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Math overflow when calculating withdrawal amount")]
    MathOverflow,
}
