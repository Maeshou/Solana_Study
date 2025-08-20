// Case 001: 単純トークン転送
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, CpiContext, mint_to, burn};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe001eKfp");

#[program]
pub mod case_001 {
    use super::*;

    pub fn execute_case_001(ctx: Context<SafeCtx001>, amount: u64) -> Result<()> {
// 単純トークン転送
        require!(ctx.accounts.authority_001.is_signer, CustomError::MissingSigner);
        require_keys_eq(ctx.accounts.vault_001.owner, ctx.program_id, CustomError::InvalidOwner);
        let initial = ctx.accounts.vault_001.to_account_info().lamports();
        **ctx.accounts.vault_001.to_account_info().try_borrow_mut_lamports()? =
            initial.checked_sub(amount).ok_or(CustomError::Underflow)?;
        let recipient_initial = ctx.accounts.recipient_001.to_account_info().lamports();
        **ctx.accounts.recipient_001.to_account_info().try_borrow_mut_lamports()? =
            recipient_initial.checked_add(amount).ok_or(CustomError::Overflow)?;
        msg!("Transferred lamports: {}", amount);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SafeCtx001<'info> {
    #[account(mut, has_one = authority)]
    pub vault_001: Account<'info, Vault001>,
    #[account(signer)]
    pub authority_001: Signer<'info>,
    #[account(mut)]
    pub recipient_001: AccountInfo<'info>,
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
