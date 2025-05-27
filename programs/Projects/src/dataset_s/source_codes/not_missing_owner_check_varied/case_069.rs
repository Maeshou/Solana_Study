// Case 69: クーポン利用
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe069eKfp");

#[program]
pub mod case_069 {
    use super::*;

// Case 69: クーポン利用
pub fn execute_safe_069(ctx: Context<SafeCtx069>, burn_qty: u64) -> Result<()> {
    require!(ctx.accounts.authority_069.is_signer, CustomError::MissingSigner);
    require_keys_eq(ctx.accounts.src_acc_069.owner, ctx.accounts.authority_069.key(), CustomError::InvalidOwner);

    token::burn(ctx.accounts.into_burn_context(), burn_qty)?;
    msg!("Burned {} tokens", burn_qty);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx069<'info> {
    #[account(mut)]
    pub vault_069: Account<'info, Vault069>,
    #[account(signer)]
    pub authority_069: Signer<'info>,
    #[account(mut)]
    pub recipient_069: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_069: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_069: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_069: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_069: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_069: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault069 {
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