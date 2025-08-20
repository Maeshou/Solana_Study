// Case 040: CPI承認
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, CpiContext, mint_to, burn};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe040eKfp");

#[program]
pub mod case_040 {
    use super::*;

    pub fn execute_case_040(ctx: Context<SafeCtx040>, amount: u64) -> Result<()> {
// CPI承認
        require!(ctx.accounts.authority_040.is_signer, CustomError::MissingSigner);
        let price = ctx.accounts[market_acc_040].listed.remove(&ctx.accounts[nft_acc_040].mint).ok_or(CustomError::InvalidOwner)?;
        **ctx.accounts.seller_040.to_account_info().try_borrow_mut_lamports()? += price;
        let old_bal = ctx.accounts[authority_040].to_account_info().lamports();
        **ctx.accounts[authority_040].to_account_info().try_borrow_mut_lamports()? =
            old_bal.checked_sub(price).ok_or(CustomError::Underflow)?;
        msg!("Purchased NFT: {:?} @ {}", ctx.accounts[nft_acc_040].mint, price);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SafeCtx040<'info> {
    #[account(mut)]
    pub nft_acc_040: Account<'info, NftAccount>,
    #[account(mut)]
    pub market_acc_040: Account<'info, MarketAccount>,
    #[account(mut)]
    pub seller_040: AccountInfo<'info>,
    #[account(signer)]
    pub authority_040: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault040 {
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
