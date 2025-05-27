// Case 29: メタデータ更新
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe029eKfp");

#[program]
pub mod case_029 {
    use super::*;

// Case 29: メタデータ更新
pub fn execute_safe_029(ctx: Context<SafeCtx029>) -> Result<()> {
    require!(ctx.accounts.authority_029.is_signer, CustomError::MissingSigner);
    let reward = ctx.accounts.claim_acc_029.calculate()?;
    ctx.accounts.claim_acc_029.balance = ctx.accounts.claim_acc_029.balance.checked_add(reward).ok_or(CustomError::Overflow)?;
    msg!("Claimed reward: {}", reward);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx029<'info> {
    #[account(mut)]
    pub vault_029: Account<'info, Vault029>,
    #[account(signer)]
    pub authority_029: Signer<'info>,
    #[account(mut)]
    pub recipient_029: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_029: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_029: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_029: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_029: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_029: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault029 {
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