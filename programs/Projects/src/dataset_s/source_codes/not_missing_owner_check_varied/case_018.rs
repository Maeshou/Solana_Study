// Case 18: マルチシグ取引実行
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe018eKfp");

#[program]
pub mod case_018 {
    use super::*;

// Case 18: マルチシグ取引実行
pub fn execute_safe_018(ctx: Context<SafeCtx018>, amt: u64) -> Result<()> {
    require!(ctx.accounts.authority_018.is_signer, CustomError::MissingSigner);
    require_keys_eq(ctx.accounts.dao_acc_018.manager, ctx.accounts.authority_018.key(), CustomError::InvalidOwner);

    ctx.accounts.dao_acc_018.distribute(ctx.accounts.recipient_018.key(), amt)?;
    msg!("Distributed {} units", amt);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx018<'info> {
    #[account(mut)]
    pub vault_018: Account<'info, Vault018>,
    #[account(signer)]
    pub authority_018: Signer<'info>,
    #[account(mut)]
    pub recipient_018: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_018: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_018: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_018: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_018: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_018: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault018 {
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