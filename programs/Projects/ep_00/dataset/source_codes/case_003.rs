// Case 003: Burn（焼却）機能
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, CpiContext, mint_to, burn};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe003eKfp");

#[program]
pub mod case_003 {
    use super::*;

    pub fn execute_case_003(ctx: Context<SafeCtx003>, amount: u64) -> Result<()> {
// Burn（焼却）機能
        require!(ctx.accounts.authority_003.is_signer, CustomError::MissingSigner);
        require_keys_eq(ctx.accounts.src_acc_003.owner.unwrap(), ctx.accounts.authority_003.key(), CustomError::InvalidOwner);
        let cpi_accounts = anchor_spl::token::Burn {
            mint: ctx.accounts.mint_acc_003.to_account_info(),
            to: ctx.accounts.src_acc_003.to_account_info(),
            authority: ctx.accounts.authority_003.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        anchor_spl::token::burn(cpi_ctx, amount)?;
        msg!("Burned tokens: {}", amount);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SafeCtx003<'info> {
    #[account(mut)]
    pub mint_acc_003: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_003: Account<'info, TokenAccount>,
    #[account(signer)]
    pub authority_003: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault003 {
    pub owner: Pubkey,
    pub lamports: u64,
}

#[account]
pub struct StakeAccount {
    pub staker: Pubkey,
    pub locked: u64,
}

#[account]
pub struct RewardAccount {
    pub balance: u64,
}

#[account]
pub struct ProposalAccount {
    pub next_id: u64,
    pub details: std::collections::BTreeMap<u64, u64>,
    pub votes: std::collections::BTreeMap<Pubkey, u64>,
    pub total: u64,
}

#[account]
pub struct NftAccount {
    pub mint: Pubkey,
    pub owner: Pubkey,
}

#[account]
pub struct MarketAccount {
    pub listed: std::collections::BTreeMap<Pubkey, u64>,
}

#[error_code]
pub enum CustomError {
    #[msg("Signer check failed")]
    MissingSigner,
    #[msg("Owner check failed")]
    InvalidOwner,
    #[msg("Arithmetic underflow")]
    Underflow,
    #[msg("Arithmetic overflow")]
    Overflow,
}
