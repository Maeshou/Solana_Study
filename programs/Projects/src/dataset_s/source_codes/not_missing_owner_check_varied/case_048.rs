// Case 48: 取引履歴照会
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe048eKfp");

#[program]
pub mod case_048 {
    use super::*;

// Case 48: 取引履歴照会
pub fn execute_safe_048(ctx: Context<SafeCtx048>, amt: u64) -> Result<()> {
    require!(ctx.accounts.authority_048.is_signer, CustomError::MissingSigner);
    require_keys_eq(ctx.accounts.dao_acc_048.manager, ctx.accounts.authority_048.key(), CustomError::InvalidOwner);

    ctx.accounts.dao_acc_048.distribute(ctx.accounts.recipient_048.key(), amt)?;
    msg!("Distributed {} units", amt);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx048<'info> {
    #[account(mut)]
    pub vault_048: Account<'info, Vault048>,
    #[account(signer)]
    pub authority_048: Signer<'info>,
    #[account(mut)]
    pub recipient_048: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_048: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_048: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_048: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_048: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_048: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault048 {
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