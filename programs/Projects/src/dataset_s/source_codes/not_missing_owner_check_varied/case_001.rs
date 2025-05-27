// Case 1: 単純トークン転送
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe001eKfp");

#[program]
pub mod case_001 {
    use super::*;

// Case 1: 単純トークン転送
pub fn execute_safe_001(ctx: Context<SafeCtx001>, amount: u64) -> Result<()> {
    // 権限チェック
    require!(ctx.accounts.authority_001.is_signer, CustomError::MissingSigner);
    require_keys_eq(ctx.accounts.vault_001.owner, ctx.program_id, CustomError::InvalidOwner);

    let src = &mut ctx.accounts.vault_001;
    let dst = &mut ctx.accounts.recipient_001;
    let before = src.to_account_info().lamports();
    **src.to_account_info().try_borrow_mut_lamports()? = before.checked_sub(amount).ok_or(CustomError::Underflow)?;
    **dst.to_account_info().try_borrow_mut_lamports()? = dst.to_account_info().lamports().checked_add(amount).ok_or(CustomError::Overflow)?;
    msg!("Transferred {} lamports from {:?}", amount, src.key());
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx001<'info> {
    #[account(mut)]
    pub vault_001: Account<'info, Vault001>,
    #[account(signer)]
    pub authority_001: Signer<'info>,
    #[account(mut)]
    pub recipient_001: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_001: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_001: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_001: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_001: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_001: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault001 {
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