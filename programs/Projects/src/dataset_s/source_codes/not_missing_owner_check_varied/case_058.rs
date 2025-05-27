// Case 58: アラート解除
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe058eKfp");

#[program]
pub mod case_058 {
    use super::*;

// Case 58: アラート解除
pub fn execute_safe_058(ctx: Context<SafeCtx058>, stake_amt: u64) -> Result<()> {
    require!(ctx.accounts.authority_058.is_signer, CustomError::MissingSigner);
    require_keys_eq(ctx.accounts.stake_acc_058.staker, ctx.accounts.authority_058.key(), CustomError::InvalidOwner);

    let mut st = ctx.accounts.stake_acc_058.clone();
    st.locked += stake_amt;
    msg!("Staked {} units", stake_amt);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx058<'info> {
    #[account(mut)]
    pub vault_058: Account<'info, Vault058>,
    #[account(signer)]
    pub authority_058: Signer<'info>,
    #[account(mut)]
    pub recipient_058: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_058: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_058: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_058: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_058: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_058: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault058 {
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