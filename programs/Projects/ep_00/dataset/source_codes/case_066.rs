// Case 066: 支払い条件設定
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, CpiContext, mint_to, burn};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe066eKfp");

#[program]
pub mod case_066 {
    use super::*;

    pub fn execute_case_066(ctx: Context<SafeCtx066>, amount: u64) -> Result<()> {
// 支払い条件設定
        require!(ctx.accounts.authority_066.is_signer, CustomError::MissingSigner);
        let reward = ctx.accounts.reward_acc_066.calculate()?;
        let old = ctx.accounts.reward_acc_066.balance;
        ctx.accounts.reward_acc_066.balance = old.checked_add(reward).ok_or(CustomError::Overflow)?;
        msg!("Reward granted: {}", reward);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SafeCtx066<'info> {
    #[account(mut)]
    pub reward_acc_066: Account<'info, RewardAccount>,
    #[account(signer)]
    pub authority_066: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault066 {
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
