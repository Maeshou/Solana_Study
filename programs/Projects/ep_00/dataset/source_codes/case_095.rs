// Case 095: オークション終了
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, CpiContext, mint_to, burn};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe095eKfp");

#[program]
pub mod case_095 {
    use super::*;

    pub fn execute_case_095(ctx: Context<SafeCtx095>, amount: u64) -> Result<()> {
// オークション終了
        require!(ctx.accounts.authority_095.is_signer, CustomError::MissingSigner);
        require_keys_eq(ctx.accounts.stake_acc_095.staker, ctx.accounts.authority_095.key(), CustomError::InvalidOwner);
        let before_stake = ctx.accounts.stake_acc_095.locked;
        ctx.accounts.stake_acc_095.locked = before_stake.checked_sub(amount).ok_or(CustomError::Underflow)?;
        **ctx.accounts.authority_095.to_account_info().try_borrow_mut_lamports()? += amount;
        msg!("Unstaked: {} units", amount);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SafeCtx095<'info> {
    #[account(mut)]
    pub stake_acc_095: Account<'info, StakeAccount>,
    #[account(signer)]
    pub authority_095: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault095 {
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
