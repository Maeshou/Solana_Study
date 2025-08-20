// 3. NFT Marketplace with Collection Verification
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

declare_id!("NFTMarketplace111111111111111111111111111111111");

#[program]
pub mod nft_marketplace {
    use super::*;
    
    pub fn list_nft(ctx: Context<ListNFT>, price: u64) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        listing.nft_mint = ctx.accounts.nft_mint.key();
        listing.seller = ctx.accounts.seller.key();
        listing.price = price;
        listing.is_active = true;
        
        // Transfer NFT to escrow
        let cpi_accounts = anchor_spl::token::Transfer {
            from: ctx.accounts.seller_token_account.to_account_info(),
            to: ctx.accounts.escrow_token_account.to_account_info(),
            authority: ctx.accounts.seller.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        anchor_spl::token::transfer(cpi_ctx, 1)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ListNFT<'info> {
    #[account(init, payer = seller, space = 8 + 200, seeds = [b"listing", nft_mint.key().as_ref()], bump)]
    pub listing: Account<'info, Listing>,
    pub nft_mint: Account<'info, Mint>,
    #[account(mut, constraint = seller_token_account.mint == nft_mint.key())]
    pub seller_token_account: Account<'info, TokenAccount>,
    #[account(mut, constraint = escrow_token_account.mint == nft_mint.key())]
    pub escrow_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub seller: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Listing {
    pub nft_mint: Pubkey,
    pub seller: Pubkey,
    pub price: u64,
    pub is_active: bool,
}
