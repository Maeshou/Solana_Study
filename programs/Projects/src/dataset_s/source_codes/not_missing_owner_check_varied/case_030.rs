// Case 30: クロスチェーンブリッジ実行
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe030eKfp");

#[program]
pub mod case_030 {
    use super::*;

// Case 30: クロスチェーンブリッジ実行
pub fn execute_safe_030(ctx: Context<SafeCtx030>, amt: u64) -> Result<()> {
    require!(ctx.accounts.authority_030.is_signer, CustomError::MissingSigner);
    require_keys_eq(ctx.accounts.dao_acc_030.manager, ctx.accounts.authority_030.key(), CustomError::InvalidOwner);

    ctx.accounts.dao_acc_030.distribute(ctx.accounts.recipient_030.key(), amt)?;
    msg!("Distributed {} units", amt);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx030<'info> {
    #[account(mut)]
    pub vault_030: Account<'info, Vault030>,
    #[account(signer)]
    pub authority_030: Signer<'info>,
    #[account(mut)]
    pub recipient_030: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_030: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_030: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_030: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_030: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_030: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault030 {
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