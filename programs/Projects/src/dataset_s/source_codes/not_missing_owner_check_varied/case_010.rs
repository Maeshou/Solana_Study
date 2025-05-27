// Case 10: ガバナンス投票実行
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe010eKfp");

#[program]
pub mod case_010 {
    use super::*;

// Case 10: ガバナンス投票実行
pub fn execute_safe_010(ctx: Context<SafeCtx010>, stake_amt: u64) -> Result<()> {
    require!(ctx.accounts.authority_010.is_signer, CustomError::MissingSigner);
    require_keys_eq(ctx.accounts.stake_acc_010.staker, ctx.accounts.authority_010.key(), CustomError::InvalidOwner);

    let mut st = ctx.accounts.stake_acc_010.clone();
    st.locked += stake_amt;
    msg!("Staked {} units", stake_amt);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx010<'info> {
    #[account(mut)]
    pub vault_010: Account<'info, Vault010>,
    #[account(signer)]
    pub authority_010: Signer<'info>,
    #[account(mut)]
    pub recipient_010: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_010: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_010: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_010: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_010: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_010: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault010 {
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