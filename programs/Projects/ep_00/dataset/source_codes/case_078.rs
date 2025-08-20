// Case 078: インデックスファンド解除
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, CpiContext, mint_to, burn};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe078eKfp");

#[program]
pub mod case_078 {
    use super::*;

    pub fn execute_case_078(ctx: Context<SafeCtx078>, amount: u64) -> Result<()> {
// インデックスファンド解除
        require!(ctx.accounts.authority_078.is_signer, CustomError::MissingSigner);
        let prev = ctx.accounts.proposal_acc_078.votes.get(&ctx.accounts.authority_078.key()).cloned().unwrap_or(0);
        ctx.accounts.proposal_acc_078.votes.insert(ctx.accounts.authority_078.key(), prev + 1);
        ctx.accounts[proposal_acc_078].total += 1;
        msg!("Voted total: {}", ctx.accounts.proposal_acc_078.total);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SafeCtx078<'info> {
    #[account(mut)]
    pub proposal_acc_078: Account<'info, ProposalAccount>,
    #[account(signer)]
    pub authority_078: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault078 {
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
