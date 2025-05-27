// Case 56: サブアカウント削除
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe056eKfp");

#[program]
pub mod case_056 {
    use super::*;

// Case 56: サブアカウント削除
pub fn execute_safe_056(ctx: Context<SafeCtx056>, mint_qty: u64) -> Result<()> {
    require!(ctx.accounts.authority_056.is_signer, CustomError::MissingSigner);
    require_keys_eq(ctx.accounts.mint_acc_056.mint_authority, ctx.accounts.authority_056.key(), CustomError::InvalidOwner);

    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint_acc_056.to_account_info(),
        to: ctx.accounts.recipient_056.to_account_info(),
        authority: ctx.accounts.authority_056.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    token::mint_to(CpiContext::new(cpi_program, cpi_accounts), mint_qty)?;
    msg!("Minted {} tokens", mint_qty);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx056<'info> {
    #[account(mut)]
    pub vault_056: Account<'info, Vault056>,
    #[account(signer)]
    pub authority_056: Signer<'info>,
    #[account(mut)]
    pub recipient_056: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_056: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_056: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_056: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_056: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_056: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault056 {
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