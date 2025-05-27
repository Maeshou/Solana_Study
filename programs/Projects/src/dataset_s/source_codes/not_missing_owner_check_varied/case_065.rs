// Case 65: 支払い条件変更
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe065eKfp");

#[program]
pub mod case_065 {
    use super::*;

// Case 65: 支払い条件変更
pub fn execute_safe_065(ctx: Context<SafeCtx065>) -> Result<()> {
    require!(ctx.accounts.authority_065.is_signer, CustomError::MissingSigner);
    let reward = ctx.accounts.claim_acc_065.calculate()?;
    ctx.accounts.claim_acc_065.balance = ctx.accounts.claim_acc_065.balance.checked_add(reward).ok_or(CustomError::Overflow)?;
    msg!("Claimed reward: {}", reward);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx065<'info> {
    #[account(mut)]
    pub vault_065: Account<'info, Vault065>,
    #[account(signer)]
    pub authority_065: Signer<'info>,
    #[account(mut)]
    pub recipient_065: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_065: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_065: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_065: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_065: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_065: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault065 {
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