// Case 028: DAOトレジャリー出金
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, CpiContext, mint_to, burn};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe028eKfp");

#[program]
pub mod case_028 {
    use super::*;

    pub fn execute_case_028(ctx: Context<SafeCtx028>, amount: u64) -> Result<()> {
// DAOトレジャリー出金
        require!(ctx.accounts.authority_028.is_signer, CustomError::MissingSigner);
        let prev = ctx.accounts.proposal_acc_028.votes.get(&ctx.accounts.authority_028.key()).cloned().unwrap_or(0);
        ctx.accounts.proposal_acc_028.votes.insert(ctx.accounts.authority_028.key(), prev + 1);
        ctx.accounts[proposal_acc_028].total += 1;
        msg!("Voted total: {}", ctx.accounts.proposal_acc_028.total);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SafeCtx028<'info> {
    #[account(mut)]
    pub proposal_acc_028: Account<'info, ProposalAccount>,
    #[account(signer)]
    pub authority_028: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault028 {
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
