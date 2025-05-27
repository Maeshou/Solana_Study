// Case 90: 担保評価更新
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe090eKfp");

#[program]
pub mod case_090 {
    use super::*;

// Case 90: 担保評価更新
pub fn execute_safe_090(ctx: Context<SafeCtx090>, amt: u64) -> Result<()> {
    require!(ctx.accounts.authority_090.is_signer, CustomError::MissingSigner);
    require_keys_eq(ctx.accounts.dao_acc_090.manager, ctx.accounts.authority_090.key(), CustomError::InvalidOwner);

    ctx.accounts.dao_acc_090.distribute(ctx.accounts.recipient_090.key(), amt)?;
    msg!("Distributed {} units", amt);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx090<'info> {
    #[account(mut)]
    pub vault_090: Account<'info, Vault090>,
    #[account(signer)]
    pub authority_090: Signer<'info>,
    #[account(mut)]
    pub recipient_090: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_090: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_090: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_090: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_090: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_090: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault090 {
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