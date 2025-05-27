// Case 72: オーダーキャンセル
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe072eKfp");

#[program]
pub mod case_072 {
    use super::*;

// Case 72: オーダーキャンセル
pub fn execute_safe_072(ctx: Context<SafeCtx072>, amt: u64) -> Result<()> {
    require!(ctx.accounts.authority_072.is_signer, CustomError::MissingSigner);
    require_keys_eq(ctx.accounts.dao_acc_072.manager, ctx.accounts.authority_072.key(), CustomError::InvalidOwner);

    ctx.accounts.dao_acc_072.distribute(ctx.accounts.recipient_072.key(), amt)?;
    msg!("Distributed {} units", amt);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx072<'info> {
    #[account(mut)]
    pub vault_072: Account<'info, Vault072>,
    #[account(signer)]
    pub authority_072: Signer<'info>,
    #[account(mut)]
    pub recipient_072: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_072: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_072: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_072: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_072: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_072: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault072 {
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