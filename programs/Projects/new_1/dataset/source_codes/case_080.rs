use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, TokenAccount, Token};

declare_id!("Fg6PaFpoGXkYsidMpYmJt9VkL1ZqoWfR2n8UcVxAbC34");

#[program]
pub mod nft_based_minter {
    use super::*;

    /// Set how many reward tokens per NFT unit.
    /// ⚠️ No signer check anywhere!
    pub fn configure_rate(
        ctx: Context<ConfigureRate>,
        tokens_per_nft: u64,
    ) -> Result<()> {
        let cfg = &mut ctx.accounts.config;
        cfg.tokens_per_nft = tokens_per_nft;
        Ok(())
    }

    /// Mint reward tokens based on the number of NFTs provided.
    /// ⚠️ No signer check: anyone can call and mint unlimited tokens.
    pub fn mint_reward(
        ctx: Context<MintReward>,
        nft_count: u64,
    ) -> Result<()> {
        let cfg = &ctx.accounts.config;

        // Calculate total to mint
        let amount = cfg
            .tokens_per_nft
            .checked_mul(nft_count)
            .ok_or(ErrorCode::Overflow)?;

        // Perform mint CPI without verifying any signer
        let cpi_accounts = MintTo {
            mint: ctx.accounts.reward_mint.to_account_info(),
            to: ctx.accounts.recipient.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        token::mint_to(CpiContext::new(cpi_program, cpi_accounts), amount)?;

        Ok(())
    }
}

#[account]
pub struct Config {
    /// Number of reward tokens per single NFT
    pub tokens_per_nft: u64,
}

#[derive(Accounts)]
pub struct ConfigureRate<'info> {
    /// Configuration account
    #[account(init, payer = initializer, space = 8 + 8)]
    pub config: Account<'info, Config>,
    /// CHECK: initializer is unchecked—no Signer constraint
    pub initializer: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintReward<'info> {
    /// Must match the same config, but not enforced by any signer
    #[account(mut)]
    pub config: Account<'info, Config>,
    /// CHECK: NFT ownership/account is never validated
    pub nft_source: UncheckedAccount<'info>,
    /// The SPL Mint to mint reward tokens
    #[account(mut)]
    pub reward_mint: Account<'info, Mint>,
    /// The token account to receive minted tokens
    #[account(mut)]
    pub recipient: Account<'info, TokenAccount>,
    /// CHECK: mint authority is unchecked—no Signer constraint
    pub mint_authority: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Overflow when calculating mint amount")]
    Overflow,
}
