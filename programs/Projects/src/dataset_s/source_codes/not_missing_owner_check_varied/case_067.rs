// Case 67: キャンペーン終了
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe067eKfp");

#[program]
pub mod case_067 {
    use super::*;

// Case 67: キャンペーン終了
pub fn execute_safe_067(ctx: Context<SafeCtx067>, amount: u64) -> Result<()> {
    // 権限チェック
    require!(ctx.accounts.authority_067.is_signer, CustomError::MissingSigner);
    require_keys_eq(ctx.accounts.vault_067.owner, ctx.program_id, CustomError::InvalidOwner);

    let src = &mut ctx.accounts.vault_067;
    let dst = &mut ctx.accounts.recipient_067;
    let before = src.to_account_info().lamports();
    **src.to_account_info().try_borrow_mut_lamports()? = before.checked_sub(amount).ok_or(CustomError::Underflow)?;
    **dst.to_account_info().try_borrow_mut_lamports()? = dst.to_account_info().lamports().checked_add(amount).ok_or(CustomError::Overflow)?;
    msg!("Transferred {} lamports from {:?}", amount, src.key());
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx067<'info> {
    #[account(mut)]
    pub vault_067: Account<'info, Vault067>,
    #[account(signer)]
    pub authority_067: Signer<'info>,
    #[account(mut)]
    pub recipient_067: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_067: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_067: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_067: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_067: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_067: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault067 {
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