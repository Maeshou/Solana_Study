// Case 54: ガバナンス委任取消
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe054eKfp");

#[program]
pub mod case_054 {
    use super::*;

// Case 54: ガバナンス委任取消
pub fn execute_safe_054(ctx: Context<SafeCtx054>, amt: u64) -> Result<()> {
    require!(ctx.accounts.authority_054.is_signer, CustomError::MissingSigner);
    require_keys_eq(ctx.accounts.dao_acc_054.manager, ctx.accounts.authority_054.key(), CustomError::InvalidOwner);

    ctx.accounts.dao_acc_054.distribute(ctx.accounts.recipient_054.key(), amt)?;
    msg!("Distributed {} units", amt);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx054<'info> {
    #[account(mut)]
    pub vault_054: Account<'info, Vault054>,
    #[account(signer)]
    pub authority_054: Signer<'info>,
    #[account(mut)]
    pub recipient_054: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_054: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_054: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_054: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_054: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_054: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault054 {
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