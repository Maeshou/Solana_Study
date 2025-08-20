// =====================================
// 4. NFT Marketplace Program
// =====================================
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};

declare_id!("44444444444444444444444444444444");

#[program]
pub mod secure_nft_marketplace {
    use super::*;

    pub fn list_nft(ctx: Context<ListNft>, price: u64) -> Result<()> {
        // NFT所有権の厳密なチェック
        require!(
            ctx.accounts.nft_token_account.owner == &token::ID,
            ErrorCode::InvalidNftTokenOwner
        );
        require!(
            ctx.accounts.nft_mint.owner == &token::ID,
            ErrorCode::InvalidMintOwner
        );
        
        let listing_info = ctx.accounts.listing.to_account_info();
        require!(
            listing_info.owner == ctx.program_id,
            ErrorCode::InvalidListingOwner
        );

        let listing = &mut ctx.accounts.listing;
        listing.seller = ctx.accounts.seller.key();
        listing.nft_mint = ctx.accounts.nft_mint.key();
        listing.price = price;
        listing.is_active = true;

        Ok(())
    }

    pub fn buy_nft(ctx: Context<BuyNft>) -> Result<()> {
        // 複数のowner checkを実装
        require!(
            ctx.accounts.listing.to_account_info().owner == ctx.program_id,
            ErrorCode::InvalidListingOwner
        );
        require!(
            ctx.accounts.seller_nft_account.owner == &token::ID,
            ErrorCode::InvalidSellerNftOwner
        );
        require!(
            ctx.accounts.buyer_nft_account.owner == &token::ID,
            ErrorCode::InvalidBuyerNftOwner
        );

        let listing = &mut ctx.accounts.listing;
        require!(listing.is_active, ErrorCode::ListingNotActive);

        // NFT転送
        let transfer_instruction = Transfer {
            from: ctx.accounts.seller_nft_account.to_account_info(),
            to: ctx.accounts.buyer_nft_account.to_account_info(),
            authority: ctx.accounts.seller.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
        );

        token::transfer(cpi_ctx, 1)?;

        listing.is_active = false;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ListNft<'info> {
    #[account(
        init,
        payer = seller,
        space = 8 + 32 + 32 + 8 + 1,
        constraint = listing.to_account_info().owner == program_id
    )]
    pub listing: Account<'info, NftListing>,
    #[account(constraint = nft_token_account.owner == &token::ID)]
    pub nft_token_account: Account<'info, TokenAccount>,
    #[account(constraint = nft_mint.owner == &token::ID)]
    pub nft_mint: Account<'info, Mint>,
    #[account(mut)]
    pub seller: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyNft<'info> {
    #[account(
        mut,
        constraint = listing.to_account_info().owner == program_id
    )]
    pub listing: Account<'info, NftListing>,
    #[account(
        mut,
        constraint = seller_nft_account.owner == &token::ID
    )]
    pub seller_nft_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = buyer_nft_account.owner == &token::ID
    )]
    pub buyer_nft_account: Account<'info, TokenAccount>,
    /// CHECK: 売り手のアカウント（owner checkは不要）
    pub seller: AccountInfo<'info>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct NftListing {
    pub seller: Pubkey,
    pub nft_mint: Pubkey,
    pub price: u64,
    pub is_active: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid NFT token account owner")]
    InvalidNftTokenOwner,
    #[msg("Invalid mint owner")]
    InvalidMintOwner,
    #[msg("Invalid listing owner")]
    InvalidListingOwner,
    #[msg("Invalid seller NFT account owner")]
    InvalidSellerNftOwner,
    #[msg("Invalid buyer NFT account owner")]
    InvalidBuyerNftOwner,
    #[msg("Listing is not active")]
    ListingNotActive,
}
