// Case 68: クーポン発行
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, MintTo, Burn, Mint, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe068eKfp");

#[program]
pub mod case_068 {
    use super::*;

// Case 68: クーポン発行
pub fn execute_safe_068(ctx: Context<SafeCtx068>, mint_qty: u64) -> Result<()> {
    require!(ctx.accounts.authority_068.is_signer, CustomError::MissingSigner);
    require_keys_eq(ctx.accounts.mint_acc_068.mint_authority, ctx.accounts.authority_068.key(), CustomError::InvalidOwner);

    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint_acc_068.to_account_info(),
        to: ctx.accounts.recipient_068.to_account_info(),
        authority: ctx.accounts.authority_068.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    token::mint_to(CpiContext::new(cpi_program, cpi_accounts), mint_qty)?;
    msg!("Minted {} tokens", mint_qty);
    Ok(())
}

}

#[derive(Accounts)]
pub struct SafeCtx068<'info> {
    #[account(mut)]
    pub vault_068: Account<'info, Vault068>,
    #[account(signer)]
    pub authority_068: Signer<'info>,
    #[account(mut)]
    pub recipient_068: AccountInfo<'info>,
    #[account(mut)]
    pub mint_acc_068: Account<'info, Mint>,
    #[account(mut)]
    pub src_acc_068: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_acc_068: Account<'info, StakeAccount>,
    #[account(mut)]
    pub claim_acc_068: Account<'info, RewardAccount>,
    #[account(mut)]
    pub dao_acc_068: Account<'info, DaoAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault068 {
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