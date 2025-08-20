// Case 052: トークンアンラップ
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, CpiContext, mint_to, burn};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe052eKfp");

#[program]
pub mod case_052 {
    use super::*;

    pub fn execute_case_052(ctx: Context<SafeCtx052>, amount: u64) -> Result<()> {
// トークンアンラップ
        require!(ctx.accounts.authority_052.is_signer, CustomError::MissingSigner);
        require_keys_eq(ctx.accounts.mint_acc_052.mint_authority.unwrap(), ctx.accounts.authority_052.key(), CustomError::InvalidOwner);
        let cpi_accounts = anchor_spl::token::MintTo {
            mint: ctx.accounts.mint_acc_052.to_account_info(),
            to: ctx.accounts.recipient_052.to_account_info(),
            authority: ctx.accounts.authority_052.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        anchor_spl::token::mint_to(cpi_ctx, amount)?;
        msg!("Minted tokens: {}", amount);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SafeCtx052<'info> {
    #[account(mut)]
    pub mint_acc_052: Account<'info, Mint>,
    #[account(signer)]
    pub authority_052: Signer<'info>,
    #[account(mut)]
    pub recipient_052: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault052 {
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
pub struct ProposalAccount {
    pub next_id: u64,
    pub details: std::collections::BTreeMap<u64, u64>,
    pub votes: std::collections::BTreeMap<Pubkey, u64>,
    pub total: u64,
}

#[account]
pub struct NftAccount {
    pub mint: Pubkey,
    pub owner: Pubkey,
}

#[account]
pub struct MarketAccount {
    pub listed: std::collections::BTreeMap<Pubkey, u64>,
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
