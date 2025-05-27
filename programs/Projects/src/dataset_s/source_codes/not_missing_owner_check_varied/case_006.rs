// Case 6: 報酬請求（Reward Claim）
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe006eKfp");

#[program]
pub mod case_006 {
    use super::*;

// Case 6: 報酬請求（Reward Claim）
pub fn execute_safe_006(ctx: Context<SafeCtx006>, amt: u64) -> Result<()> {
    require!(ctx.accounts.authority_006.is_signer, CustomError::MissingSigner);
    require_keys_eq(ctx.accounts.dao_acc_006.manager, ctx.accounts.authority_006.key(), CustomError::InvalidOwner);

    ctx.accounts.dao_acc_006.distribute(ctx.accounts.recipient_006.key(), amt)?;
    msg!("Distributed {} units", amt);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx006<'info> {
    #[account(mut)]
    pub vault_006: Account<'info, Vault006>,
    #[account(signer)]
    pub authority_006: Signer<'info>,
    #[account(mut)]
    pub recipient_006: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_006: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_006: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_006: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_006: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_006: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault006 {
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