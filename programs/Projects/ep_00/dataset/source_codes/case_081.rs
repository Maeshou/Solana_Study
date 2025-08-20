// Case 081: ペイアウトスケジュール更新
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, CpiContext, mint_to, burn};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe081eKfp");

#[program]
pub mod case_081 {
    use super::*;

    pub fn execute_case_081(ctx: Context<SafeCtx081>, amount: u64) -> Result<()> {
// ペイアウトスケジュール更新
        require!(ctx.accounts.authority_081.is_signer, CustomError::MissingSigner);
        require_keys_eq(ctx.accounts.vault_081.owner, ctx.program_id, CustomError::InvalidOwner);
        let initial = ctx.accounts.vault_081.to_account_info().lamports();
        **ctx.accounts.vault_081.to_account_info().try_borrow_mut_lamports()? =
            initial.checked_sub(amount).ok_or(CustomError::Underflow)?;
        let recipient_initial = ctx.accounts.recipient_081.to_account_info().lamports();
        **ctx.accounts.recipient_081.to_account_info().try_borrow_mut_lamports()? =
            recipient_initial.checked_add(amount).ok_or(CustomError::Overflow)?;
        msg!("Transferred lamports: {}", amount);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SafeCtx081<'info> {
    #[account(mut, has_one = authority)]
    pub vault_081: Account<'info, Vault081>,
    #[account(signer)]
    pub authority_081: Signer<'info>,
    #[account(mut)]
    pub recipient_081: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault081 {
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
