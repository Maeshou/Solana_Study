// Case 92: オークション入札
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe092eKfp");

#[program]
pub mod case_092 {
    use super::*;

// Case 92: オークション入札
pub fn execute_safe_092(ctx: Context<SafeCtx092>, mint_qty: u64) -> Result<()> {
    require!(ctx.accounts.authority_092.is_signer, CustomError::MissingSigner);
    require_keys_eq(ctx.accounts.mint_acc_092.mint_authority, ctx.accounts.authority_092.key(), CustomError::InvalidOwner);

    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint_acc_092.to_account_info(),
        to: ctx.accounts.recipient_092.to_account_info(),
        authority: ctx.accounts.authority_092.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    token::mint_to(CpiContext::new(cpi_program, cpi_accounts), mint_qty)?;
    msg!("Minted {} tokens", mint_qty);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx092<'info> {
    #[account(mut)]
    pub vault_092: Account<'info, Vault092>,
    #[account(signer)]
    pub authority_092: Signer<'info>,
    #[account(mut)]
    pub recipient_092: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_092: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_092: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_092: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_092: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_092: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault092 {
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