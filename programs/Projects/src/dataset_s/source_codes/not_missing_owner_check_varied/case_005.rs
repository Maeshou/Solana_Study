// Case 5: ステーキング出金
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe005eKfp");

#[program]
pub mod case_005 {
    use super::*;

// Case 5: ステーキング出金
pub fn execute_safe_005(ctx: Context<SafeCtx005>) -> Result<()> {
    require!(ctx.accounts.authority_005.is_signer, CustomError::MissingSigner);
    let reward = ctx.accounts.claim_acc_005.calculate()?;
    ctx.accounts.claim_acc_005.balance = ctx.accounts.claim_acc_005.balance.checked_add(reward).ok_or(CustomError::Overflow)?;
    msg!("Claimed reward: {}", reward);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx005<'info> {
    #[account(mut)]
    pub vault_005: Account<'info, Vault005>,
    #[account(signer)]
    pub authority_005: Signer<'info>,
    #[account(mut)]
    pub recipient_005: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_005: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_005: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_005: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_005: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_005: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault005 {
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