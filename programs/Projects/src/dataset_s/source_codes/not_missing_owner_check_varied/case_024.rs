// Case 24: 定期購読支払い
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe024eKfp");

#[program]
pub mod case_024 {
    use super::*;

// Case 24: 定期購読支払い
pub fn execute_safe_024(ctx: Context<SafeCtx024>, amt: u64) -> Result<()> {
    require!(ctx.accounts.authority_024.is_signer, CustomError::MissingSigner);
    require_keys_eq(ctx.accounts.dao_acc_024.manager, ctx.accounts.authority_024.key(), CustomError::InvalidOwner);

    ctx.accounts.dao_acc_024.distribute(ctx.accounts.recipient_024.key(), amt)?;
    msg!("Distributed {} units", amt);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx024<'info> {
    #[account(mut)]
    pub vault_024: Account<'info, Vault024>,
    #[account(signer)]
    pub authority_024: Signer<'info>,
    #[account(mut)]
    pub recipient_024: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_024: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_024: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_024: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_024: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_024: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault024 {
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