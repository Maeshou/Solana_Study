// Case 059: アラート設定
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, CpiContext, mint_to, burn};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe059eKfp");

#[program]
pub mod case_059 {
    use super::*;

    pub fn execute_case_059(ctx: Context<SafeCtx059>, amount: u64) -> Result<()> {
// アラート設定
        require!(ctx.accounts.authority_059.is_signer, CustomError::MissingSigner);
        require_keys_eq(ctx.accounts.nft_acc_059.owner, ctx.accounts.authority_059.key(), CustomError::InvalidOwner);
        ctx.accounts[market_acc_059].listed.insert(ctx.accounts[nft_acc_059].mint, amount);
        msg!("NFT listed: {:?} for {}", ctx.accounts.nft_acc_059.mint, amount);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SafeCtx059<'info> {
    #[account(mut)]
    pub nft_acc_059: Account<'info, NftAccount>,
    #[account(mut)]
    pub market_acc_059: Account<'info, MarketAccount>,
    #[account(signer)]
    pub authority_059: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault059 {
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
