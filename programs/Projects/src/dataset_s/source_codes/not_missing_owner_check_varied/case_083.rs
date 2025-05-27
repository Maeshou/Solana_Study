// Case 83: スナップショット復元
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe083eKfp");

#[program]
pub mod case_083 {
    use super::*;

// Case 83: スナップショット復元
pub fn execute_safe_083(ctx: Context<SafeCtx083>) -> Result<()> {
    require!(ctx.accounts.authority_083.is_signer, CustomError::MissingSigner);
    let reward = ctx.accounts.claim_acc_083.calculate()?;
    ctx.accounts.claim_acc_083.balance = ctx.accounts.claim_acc_083.balance.checked_add(reward).ok_or(CustomError::Overflow)?;
    msg!("Claimed reward: {}", reward);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx083<'info> {
    #[account(mut)]
    pub vault_083: Account<'info, Vault083>,
    #[account(signer)]
    pub authority_083: Signer<'info>,
    #[account(mut)]
    pub recipient_083: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_083: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_083: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_083: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_083: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_083: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault083 {
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