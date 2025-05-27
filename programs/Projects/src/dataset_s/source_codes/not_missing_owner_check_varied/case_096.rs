// Case 96: シークレット鍵登録
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe096eKfp");

#[program]
pub mod case_096 {
    use super::*;

// Case 96: シークレット鍵登録
pub fn execute_safe_096(ctx: Context<SafeCtx096>, amt: u64) -> Result<()> {
    require!(ctx.accounts.authority_096.is_signer, CustomError::MissingSigner);
    require_keys_eq(ctx.accounts.dao_acc_096.manager, ctx.accounts.authority_096.key(), CustomError::InvalidOwner);

    ctx.accounts.dao_acc_096.distribute(ctx.accounts.recipient_096.key(), amt)?;
    msg!("Distributed {} units", amt);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx096<'info> {
    #[account(mut)]
    pub vault_096: Account<'info, Vault096>,
    #[account(signer)]
    pub authority_096: Signer<'info>,
    #[account(mut)]
    pub recipient_096: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_096: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_096: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_096: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_096: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_096: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault096 {
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