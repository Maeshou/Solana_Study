// Case 52: ペナルティ適用
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe052eKfp");

#[program]
pub mod case_052 {
    use super::*;

// Case 52: ペナルティ適用
pub fn execute_safe_052(ctx: Context<SafeCtx052>, stake_amt: u64) -> Result<()> {
    require!(ctx.accounts.authority_052.is_signer, CustomError::MissingSigner);
    require_keys_eq(ctx.accounts.stake_acc_052.staker, ctx.accounts.authority_052.key(), CustomError::InvalidOwner);

    let mut st = ctx.accounts.stake_acc_052.clone();
    st.locked += stake_amt;
    msg!("Staked {} units", stake_amt);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx052<'info> {
    #[account(mut)]
    pub vault_052: Account<'info, Vault052>,
    #[account(signer)]
    pub authority_052: Signer<'info>,
    #[account(mut)]
    pub recipient_052: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_052: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_052: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_052: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_052: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_052: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault052 {
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