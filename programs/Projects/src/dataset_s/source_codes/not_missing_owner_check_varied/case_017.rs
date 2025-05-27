// Case 17: マルチシグ提案承認
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe017eKfp");

#[program]
pub mod case_017 {
    use super::*;

// Case 17: マルチシグ提案承認
pub fn execute_safe_017(ctx: Context<SafeCtx017>) -> Result<()> {
    require!(ctx.accounts.authority_017.is_signer, CustomError::MissingSigner);
    let reward = ctx.accounts.claim_acc_017.calculate()?;
    ctx.accounts.claim_acc_017.balance = ctx.accounts.claim_acc_017.balance.checked_add(reward).ok_or(CustomError::Overflow)?;
    msg!("Claimed reward: {}", reward);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx017<'info> {
    #[account(mut)]
    pub vault_017: Account<'info, Vault017>,
    #[account(signer)]
    pub authority_017: Signer<'info>,
    #[account(mut)]
    pub recipient_017: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_017: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_017: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_017: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_017: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_017: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault017 {
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