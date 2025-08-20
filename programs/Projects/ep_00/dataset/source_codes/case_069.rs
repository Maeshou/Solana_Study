// Case 069: キャンペーン終了
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, CpiContext, mint_to, burn};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe069eKfp");

#[program]
pub mod case_069 {
    use super::*;

    pub fn execute_case_069(ctx: Context<SafeCtx069>, amount: u64) -> Result<()> {
// キャンペーン終了
        require!(ctx.accounts.authority_069.is_signer, CustomError::MissingSigner);
        require_keys_eq(ctx.accounts.nft_acc_069.owner, ctx.accounts.authority_069.key(), CustomError::InvalidOwner);
        ctx.accounts[market_acc_069].listed.insert(ctx.accounts[nft_acc_069].mint, amount);
        msg!("NFT listed: {:?} for {}", ctx.accounts.nft_acc_069.mint, amount);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SafeCtx069<'info> {
    #[account(mut)]
    pub nft_acc_069: Account<'info, NftAccount>,
    #[account(mut)]
    pub market_acc_069: Account<'info, MarketAccount>,
    #[account(signer)]
    pub authority_069: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault069 {
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
