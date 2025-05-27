// Case 39: 支払いチャネルクローズ
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe039eKfp");

#[program]
pub mod case_039 {
    use super::*;

// Case 39: 支払いチャネルクローズ
pub fn execute_safe_039(ctx: Context<SafeCtx039>, burn_qty: u64) -> Result<()> {
    require!(ctx.accounts.authority_039.is_signer, CustomError::MissingSigner);
    require_keys_eq(ctx.accounts.src_acc_039.owner, ctx.accounts.authority_039.key(), CustomError::InvalidOwner);

    token::burn(ctx.accounts.into_burn_context(), burn_qty)?;
    msg!("Burned {} tokens", burn_qty);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx039<'info> {
    #[account(mut)]
    pub vault_039: Account<'info, Vault039>,
    #[account(signer)]
    pub authority_039: Signer<'info>,
    #[account(mut)]
    pub recipient_039: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_039: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_039: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_039: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_039: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_039: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault039 {
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