// Case 32: 借入清算
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe032eKfp");

#[program]
pub mod case_032 {
    use super::*;

// Case 32: 借入清算
pub fn execute_safe_032(ctx: Context<SafeCtx032>, mint_qty: u64) -> Result<()> {
    require!(ctx.accounts.authority_032.is_signer, CustomError::MissingSigner);
    require_keys_eq(ctx.accounts.mint_acc_032.mint_authority, ctx.accounts.authority_032.key(), CustomError::InvalidOwner);

    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint_acc_032.to_account_info(),
        to: ctx.accounts.recipient_032.to_account_info(),
        authority: ctx.accounts.authority_032.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    token::mint_to(CpiContext::new(cpi_program, cpi_accounts), mint_qty)?;
    msg!("Minted {} tokens", mint_qty);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx032<'info> {
    #[account(mut)]
    pub vault_032: Account<'info, Vault032>,
    #[account(signer)]
    pub authority_032: Signer<'info>,
    #[account(mut)]
    pub recipient_032: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_032: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_032: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_032: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_032: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_032: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault032 {
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