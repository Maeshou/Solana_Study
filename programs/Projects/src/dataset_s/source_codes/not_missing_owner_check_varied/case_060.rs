// Case 60: ユーザー名変更
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe060eKfp");

#[program]
pub mod case_060 {
    use super::*;

// Case 60: ユーザー名変更
pub fn execute_safe_060(ctx: Context<SafeCtx060>, amt: u64) -> Result<()> {
    require!(ctx.accounts.authority_060.is_signer, CustomError::MissingSigner);
    require_keys_eq(ctx.accounts.dao_acc_060.manager, ctx.accounts.authority_060.key(), CustomError::InvalidOwner);

    ctx.accounts.dao_acc_060.distribute(ctx.accounts.recipient_060.key(), amt)?;
    msg!("Distributed {} units", amt);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx060<'info> {
    #[account(mut)]
    pub vault_060: Account<'info, Vault060>,
    #[account(signer)]
    pub authority_060: Signer<'info>,
    #[account(mut)]
    pub recipient_060: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_060: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_060: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_060: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_060: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_060: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault060 {
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