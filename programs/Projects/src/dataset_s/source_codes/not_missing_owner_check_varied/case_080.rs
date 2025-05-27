// Case 80: 期間ロック解除
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe080eKfp");

#[program]
pub mod case_080 {
    use super::*;

// Case 80: 期間ロック解除
pub fn execute_safe_080(ctx: Context<SafeCtx080>, mint_qty: u64) -> Result<()> {
    require!(ctx.accounts.authority_080.is_signer, CustomError::MissingSigner);
    require_keys_eq(ctx.accounts.mint_acc_080.mint_authority, ctx.accounts.authority_080.key(), CustomError::InvalidOwner);

    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint_acc_080.to_account_info(),
        to: ctx.accounts.recipient_080.to_account_info(),
        authority: ctx.accounts.authority_080.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    token::mint_to(CpiContext::new(cpi_program, cpi_accounts), mint_qty)?;
    msg!("Minted {} tokens", mint_qty);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx080<'info> {
    #[account(mut)]
    pub vault_080: Account<'info, Vault080>,
    #[account(signer)]
    pub authority_080: Signer<'info>,
    #[account(mut)]
    pub recipient_080: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_080: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_080: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_080: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_080: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_080: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault080 {
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