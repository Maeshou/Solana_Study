// Case 064: メタデータ削除
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, CpiContext, mint_to, burn};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe064eKfp");

#[program]
pub mod case_064 {
    use super::*;

    pub fn execute_case_064(ctx: Context<SafeCtx064>, amount: u64) -> Result<()> {
// メタデータ削除
        require!(ctx.accounts.authority_064.is_signer, CustomError::MissingSigner);
        require_keys_eq(ctx.accounts.stake_acc_064.staker, ctx.accounts.authority_064.key(), CustomError::InvalidOwner);
        let before_stake = ctx.accounts.stake_acc_064.locked;
        ctx.accounts.stake_acc_064.locked = before_stake.checked_add(amount).ok_or(CustomError::Overflow)?;
        msg!("Staked: {} units", amount);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SafeCtx064<'info> {
    #[account(mut)]
    pub stake_acc_064: Account<'info, StakeAccount>,
    #[account(signer)]
    pub authority_064: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault064 {
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
