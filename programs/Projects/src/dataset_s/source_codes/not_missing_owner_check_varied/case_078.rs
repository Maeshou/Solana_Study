// Case 78: 利益分配設定変更
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe078eKfp");

#[program]
pub mod case_078 {
    use super::*;

// Case 78: 利益分配設定変更
pub fn execute_safe_078(ctx: Context<SafeCtx078>, amt: u64) -> Result<()> {
    require!(ctx.accounts.authority_078.is_signer, CustomError::MissingSigner);
    require_keys_eq(ctx.accounts.dao_acc_078.manager, ctx.accounts.authority_078.key(), CustomError::InvalidOwner);

    ctx.accounts.dao_acc_078.distribute(ctx.accounts.recipient_078.key(), amt)?;
    msg!("Distributed {} units", amt);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx078<'info> {
    #[account(mut)]
    pub vault_078: Account<'info, Vault078>,
    #[account(signer)]
    pub authority_078: Signer<'info>,
    #[account(mut)]
    pub recipient_078: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_078: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_078: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_078: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_078: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_078: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault078 {
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