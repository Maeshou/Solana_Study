// Case 75: インデックスファンド作成
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe075eKfp");

#[program]
pub mod case_075 {
    use super::*;

// Case 75: インデックスファンド作成
pub fn execute_safe_075(ctx: Context<SafeCtx075>, burn_qty: u64) -> Result<()> {
    require!(ctx.accounts.authority_075.is_signer, CustomError::MissingSigner);
    require_keys_eq(ctx.accounts.src_acc_075.owner, ctx.accounts.authority_075.key(), CustomError::InvalidOwner);

    token::burn(ctx.accounts.into_burn_context(), burn_qty)?;
    msg!("Burned {} tokens", burn_qty);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx075<'info> {
    #[account(mut)]
    pub vault_075: Account<'info, Vault075>,
    #[account(signer)]
    pub authority_075: Signer<'info>,
    #[account(mut)]
    pub recipient_075: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_075: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_075: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_075: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_075: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_075: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault075 {
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