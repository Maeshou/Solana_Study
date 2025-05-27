// Case 84: プロフィール写真変更
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe084eKfp");

#[program]
pub mod case_084 {
    use super::*;

// Case 84: プロフィール写真変更
pub fn execute_safe_084(ctx: Context<SafeCtx084>, amt: u64) -> Result<()> {
    require!(ctx.accounts.authority_084.is_signer, CustomError::MissingSigner);
    require_keys_eq(ctx.accounts.dao_acc_084.manager, ctx.accounts.authority_084.key(), CustomError::InvalidOwner);

    ctx.accounts.dao_acc_084.distribute(ctx.accounts.recipient_084.key(), amt)?;
    msg!("Distributed {} units", amt);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx084<'info> {
    #[account(mut)]
    pub vault_084: Account<'info, Vault084>,
    #[account(signer)]
    pub authority_084: Signer<'info>,
    #[account(mut)]
    pub recipient_084: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_084: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_084: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_084: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_084: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_084: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault084 {
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