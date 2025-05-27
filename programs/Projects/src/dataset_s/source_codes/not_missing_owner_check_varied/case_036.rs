// Case 36: 任意アドレス送金
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe036eKfp");

#[program]
pub mod case_036 {
    use super::*;

// Case 36: 任意アドレス送金
pub fn execute_safe_036(ctx: Context<SafeCtx036>, amt: u64) -> Result<()> {
    require!(ctx.accounts.authority_036.is_signer, CustomError::MissingSigner);
    require_keys_eq(ctx.accounts.dao_acc_036.manager, ctx.accounts.authority_036.key(), CustomError::InvalidOwner);

    ctx.accounts.dao_acc_036.distribute(ctx.accounts.recipient_036.key(), amt)?;
    msg!("Distributed {} units", amt);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx036<'info> {
    #[account(mut)]
    pub vault_036: Account<'info, Vault036>,
    #[account(signer)]
    pub authority_036: Signer<'info>,
    #[account(mut)]
    pub recipient_036: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_036: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_036: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_036: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_036: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_036: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault036 {
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