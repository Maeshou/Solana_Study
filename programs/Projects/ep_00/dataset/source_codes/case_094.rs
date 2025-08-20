// Case 094: オークション入札
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, CpiContext, mint_to, burn};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe094eKfp");

#[program]
pub mod case_094 {
    use super::*;

    pub fn execute_case_094(ctx: Context<SafeCtx094>, amount: u64) -> Result<()> {
// オークション入札
        require!(ctx.accounts.authority_094.is_signer, CustomError::MissingSigner);
        require_keys_eq(ctx.accounts.stake_acc_094.staker, ctx.accounts.authority_094.key(), CustomError::InvalidOwner);
        let before_stake = ctx.accounts.stake_acc_094.locked;
        ctx.accounts.stake_acc_094.locked = before_stake.checked_add(amount).ok_or(CustomError::Overflow)?;
        msg!("Staked: {} units", amount);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SafeCtx094<'info> {
    #[account(mut)]
    pub stake_acc_094: Account<'info, StakeAccount>,
    #[account(signer)]
    pub authority_094: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault094 {
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
