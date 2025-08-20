// Case 100: ライフサイクルイベントトリガー
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, CpiContext, mint_to, burn};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe100eKfp");

#[program]
pub mod case_100 {
    use super::*;

    pub fn execute_case_100(ctx: Context<SafeCtx100>, amount: u64) -> Result<()> {
// ライフサイクルイベントトリガー
        require!(ctx.accounts.authority_100.is_signer, CustomError::MissingSigner);
        let price = ctx.accounts[market_acc_100].listed.remove(&ctx.accounts[nft_acc_100].mint).ok_or(CustomError::InvalidOwner)?;
        **ctx.accounts.seller_100.to_account_info().try_borrow_mut_lamports()? += price;
        let old_bal = ctx.accounts[authority_100].to_account_info().lamports();
        **ctx.accounts[authority_100].to_account_info().try_borrow_mut_lamports()? =
            old_bal.checked_sub(price).ok_or(CustomError::Underflow)?;
        msg!("Purchased NFT: {:?} @ {}", ctx.accounts[nft_acc_100].mint, price);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SafeCtx100<'info> {
    #[account(mut)]
    pub nft_acc_100: Account<'info, NftAccount>,
    #[account(mut)]
    pub market_acc_100: Account<'info, MarketAccount>,
    #[account(mut)]
    pub seller_100: AccountInfo<'info>,
    #[account(signer)]
    pub authority_100: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault100 {
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
