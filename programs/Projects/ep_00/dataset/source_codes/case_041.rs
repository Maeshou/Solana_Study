// Case 041: PDA再割当
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, CpiContext, mint_to, burn};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe041eKfp");

#[program]
pub mod case_041 {
    use super::*;

    pub fn execute_case_041(ctx: Context<SafeCtx041>, amount: u64) -> Result<()> {
// PDA再割当
        require!(ctx.accounts.authority_041.is_signer, CustomError::MissingSigner);
        require_keys_eq(ctx.accounts.vault_041.owner, ctx.program_id, CustomError::InvalidOwner);
        let initial = ctx.accounts.vault_041.to_account_info().lamports();
        **ctx.accounts.vault_041.to_account_info().try_borrow_mut_lamports()? =
            initial.checked_sub(amount).ok_or(CustomError::Underflow)?;
        let recipient_initial = ctx.accounts.recipient_041.to_account_info().lamports();
        **ctx.accounts.recipient_041.to_account_info().try_borrow_mut_lamports()? =
            recipient_initial.checked_add(amount).ok_or(CustomError::Overflow)?;
        msg!("Transferred lamports: {}", amount);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SafeCtx041<'info> {
    #[account(mut, has_one = authority)]
    pub vault_041: Account<'info, Vault041>,
    #[account(signer)]
    pub authority_041: Signer<'info>,
    #[account(mut)]
    pub recipient_041: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault041 {
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
