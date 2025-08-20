// Case 020: プログラムアップグレード
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, CpiContext, mint_to, burn};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSafe020eKfp");

#[program]
pub mod case_020 {
    use super::*;

    pub fn execute_case_020(ctx: Context<SafeCtx020>, amount: u64) -> Result<()> {
// プログラムアップグレード
        require!(ctx.accounts.authority_020.is_signer, CustomError::MissingSigner);
        let price = ctx.accounts[market_acc_020].listed.remove(&ctx.accounts[nft_acc_020].mint).ok_or(CustomError::InvalidOwner)?;
        **ctx.accounts.seller_020.to_account_info().try_borrow_mut_lamports()? += price;
        let old_bal = ctx.accounts[authority_020].to_account_info().lamports();
        **ctx.accounts[authority_020].to_account_info().try_borrow_mut_lamports()? =
            old_bal.checked_sub(price).ok_or(CustomError::Underflow)?;
        msg!("Purchased NFT: {:?} @ {}", ctx.accounts[nft_acc_020].mint, price);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SafeCtx020<'info> {
    #[account(mut)]
    pub nft_acc_020: Account<'info, NftAccount>,
    #[account(mut)]
    pub market_acc_020: Account<'info, MarketAccount>,
    #[account(mut)]
    pub seller_020: AccountInfo<'info>,
    #[account(signer)]
    pub authority_020: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault020 {
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
