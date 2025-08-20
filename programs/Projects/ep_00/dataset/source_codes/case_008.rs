// Case 008: レンディング返済
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, CpiContext, mint_to, burn};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe008eKfp");

#[program]
pub mod case_008 {
    use super::*;

    pub fn execute_case_008(ctx: Context<SafeCtx008>, amount: u64) -> Result<()> {
// レンディング返済
        require!(ctx.accounts.authority_008.is_signer, CustomError::MissingSigner);
        let prev = ctx.accounts.proposal_acc_008.votes.get(&ctx.accounts.authority_008.key()).cloned().unwrap_or(0);
        ctx.accounts.proposal_acc_008.votes.insert(ctx.accounts.authority_008.key(), prev + 1);
        ctx.accounts[proposal_acc_008].total += 1;
        msg!("Voted total: {}", ctx.accounts.proposal_acc_008.total);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SafeCtx008<'info> {
    #[account(mut)]
    pub proposal_acc_008: Account<'info, ProposalAccount>,
    #[account(signer)]
    pub authority_008: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault008 {
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
