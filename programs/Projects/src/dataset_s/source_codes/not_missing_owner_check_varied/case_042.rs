// Case 42: ホワイトリスト管理
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe042eKfp");

#[program]
pub mod case_042 {
    use super::*;

// Case 42: ホワイトリスト管理
pub fn execute_safe_042(ctx: Context<SafeCtx042>, amt: u64) -> Result<()> {
    require!(ctx.accounts.authority_042.is_signer, CustomError::MissingSigner);
    require_keys_eq(ctx.accounts.dao_acc_042.manager, ctx.accounts.authority_042.key(), CustomError::InvalidOwner);

    ctx.accounts.dao_acc_042.distribute(ctx.accounts.recipient_042.key(), amt)?;
    msg!("Distributed {} units", amt);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx042<'info> {
    #[account(mut)]
    pub vault_042: Account<'info, Vault042>,
    #[account(signer)]
    pub authority_042: Signer<'info>,
    #[account(mut)]
    pub recipient_042: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_042: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_042: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_042: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_042: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_042: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault042 {
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