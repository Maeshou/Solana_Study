// Case 017: マルチシグ提案承認
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, CpiContext, mint_to, burn};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe017eKfp");

#[program]
pub mod case_017 {
    use super::*;

    pub fn execute_case_017(ctx: Context<SafeCtx017>, amount: u64) -> Result<()> {
// マルチシグ提案承認
        require!(ctx.accounts.authority_017.is_signer, CustomError::MissingSigner);
        let id = ctx.accounts.proposal_acc_017.next_id;
        ctx.accounts.proposal_acc_017.next_id = id.checked_add(1).ok_or(CustomError::Overflow)?;
        ctx.accounts.proposal_acc_017.details.insert(id, 0);
        msg!("Proposal ID created: {}", id);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SafeCtx017<'info> {
    #[account(mut)]
    pub proposal_acc_017: Account<'info, ProposalAccount>,
    #[account(signer)]
    pub authority_017: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault017 {
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
