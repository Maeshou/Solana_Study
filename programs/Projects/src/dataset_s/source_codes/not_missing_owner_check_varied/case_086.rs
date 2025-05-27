// Case 86: KYC情報更新
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe086eKfp");

#[program]
pub mod case_086 {
    use super::*;

// Case 86: KYC情報更新
pub fn execute_safe_086(ctx: Context<SafeCtx086>, mint_qty: u64) -> Result<()> {
    require!(ctx.accounts.authority_086.is_signer, CustomError::MissingSigner);
    require_keys_eq(ctx.accounts.mint_acc_086.mint_authority, ctx.accounts.authority_086.key(), CustomError::InvalidOwner);

    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint_acc_086.to_account_info(),
        to: ctx.accounts.recipient_086.to_account_info(),
        authority: ctx.accounts.authority_086.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    token::mint_to(CpiContext::new(cpi_program, cpi_accounts), mint_qty)?;
    msg!("Minted {} tokens", mint_qty);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx086<'info> {
    #[account(mut)]
    pub vault_086: Account<'info, Vault086>,
    #[account(signer)]
    pub authority_086: Signer<'info>,
    #[account(mut)]
    pub recipient_086: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_086: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_086: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_086: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_086: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_086: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault086 {
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