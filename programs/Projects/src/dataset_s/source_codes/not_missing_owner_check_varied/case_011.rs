// Case 11: NFTマーケット出品
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe011eKfp");

#[program]
pub mod case_011 {
    use super::*;

// Case 11: NFTマーケット出品
pub fn execute_safe_011(ctx: Context<SafeCtx011>) -> Result<()> {
    require!(ctx.accounts.authority_011.is_signer, CustomError::MissingSigner);
    let reward = ctx.accounts.claim_acc_011.calculate()?;
    ctx.accounts.claim_acc_011.balance = ctx.accounts.claim_acc_011.balance.checked_add(reward).ok_or(CustomError::Overflow)?;
    msg!("Claimed reward: {}", reward);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx011<'info> {
    #[account(mut)]
    pub vault_011: Account<'info, Vault011>,
    #[account(signer)]
    pub authority_011: Signer<'info>,
    #[account(mut)]
    pub recipient_011: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_011: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_011: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_011: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_011: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_011: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault011 {
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
pub struct DaoAccount {
    pub manager: Pubkey,
    pub total: u64,
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