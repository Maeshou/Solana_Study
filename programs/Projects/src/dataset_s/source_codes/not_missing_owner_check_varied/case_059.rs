// Case 59: ログレベル変更
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe059eKfp");

#[program]
pub mod case_059 {
    use super::*;

// Case 59: ログレベル変更
pub fn execute_safe_059(ctx: Context<SafeCtx059>) -> Result<()> {
    require!(ctx.accounts.authority_059.is_signer, CustomError::MissingSigner);
    let reward = ctx.accounts.claim_acc_059.calculate()?;
    ctx.accounts.claim_acc_059.balance = ctx.accounts.claim_acc_059.balance.checked_add(reward).ok_or(CustomError::Overflow)?;
    msg!("Claimed reward: {}", reward);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx059<'info> {
    #[account(mut)]
    pub vault_059: Account<'info, Vault059>,
    #[account(signer)]
    pub authority_059: Signer<'info>,
    #[account(mut)]
    pub recipient_059: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_059: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_059: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_059: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_059: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_059: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault059 {
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