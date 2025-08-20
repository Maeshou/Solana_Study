use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpZET1234567890abcdefGHIJKL");

#[program]
pub mod nft_marketplace {
    use super::*;

    /// NFT 出品
    pub fn create_listing(
        ctx: Context<CreateListing>,
        price: u64,
    ) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        listing.seller = *ctx.accounts.seller.key;
        listing.mint   = ctx.accounts.mint.key();
        listing.price  = price;

        // トークン転送 (seller が署名者なので通常 CPI で OK)
        let cpi_accounts = Transfer {
            from: ctx.accounts.seller_nft_account.to_account_info(),
            to:   ctx.accounts.escrow_nft_account.to_account_info(),
            authority: ctx.accounts.seller.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        transfer(cpi_ctx, 1)?;
        Ok(())
    }

    /// NFT 購入
    pub fn buy(
        ctx: Context<BuyNft>,
    ) -> Result<()> {
        let listing = &ctx.accounts.listing;
        let seller  = &ctx.accounts.seller;
        let buyer   = &ctx.accounts.buyer;

        // 自己購入禁止
        require!(listing.seller != *buyer.key, MarketplaceError::SelfPurchase);

        // Lamports の安全移動
        **buyer.to_account_info().try_borrow_mut_lamports()? =
            buyer.to_account_info().lamports()
                .checked_sub(listing.price)
                .ok_or(MarketplaceError::InsufficientFunds)?;
        **seller.to_account_info().try_borrow_mut_lamports()? =
            seller.to_account_info().lamports()
                .checked_add(listing.price)
                .ok_or(MarketplaceError::Overflow)?;

        // エスクローアカウントが本物の PDA か検証
        // → seeds＋bump で listing PDA と一致するかチェック
        let expected_escrow = Pubkey::find_program_address(
            &[b"listing", listing.mint.as_ref()],
            ctx.program_id
        ).0;
        require!(
            ctx.accounts.escrow_nft_account.key() == expected_escrow,
            MarketplaceError::MismatchEscrow
        );

        // NFT の返却
        let cpi_accounts = Transfer {
            from:      ctx.accounts.escrow_nft_account.to_account_info(),
            to:        ctx.accounts.buyer_nft_account.to_account_info(),
            authority: ctx.accounts.listing.to_account_info(),
        };
        let seeds = &[b"listing", listing.mint.as_ref(), &[listing.bump]];
        let signer = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            signer,
        );
        transfer(cpi_ctx, 1)?;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(price: u64, bump: u8)]
pub struct CreateListing<'info> {
    #[account(
        init,
        seeds = [b"listing", mint.key().as_ref()],
        bump = bump,
        payer = seller,
        space = 8 + 32 + 32 + 8 + 1,
    )]
    pub listing: Account<'info, Listing>,

    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        mut,
        constraint = seller_nft_account.owner == seller.key()
            && seller_nft_account.mint == mint.key(),
        error = MarketplaceError::Unauthorized
    )]
    pub seller_nft_account: Account<'info, TokenAccount>,

    #[account(
        init,
        seeds = [b"listing", mint.key().as_ref()],
        bump = bump,
        token::mint = mint,
        token::authority = listing,
        payer = seller
    )]
    pub escrow_nft_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct BuyNft<'info> {
    #[account(
        mut,
        seeds = [b"listing", listing.mint.as_ref()],
        bump = listing.bump,
        has_one = seller @ MarketplaceError::Unauthorized
    )]
    pub listing: Account<'info, Listing>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    /// CHECK: lamports 送金先（署名不要）
    #[account(mut)]
    pub seller: AccountInfo<'info>,

    #[account(
        mut,
        constraint = buyer_nft_account.owner == buyer.key()
            && buyer_nft_account.mint == listing.mint,
        error = MarketplaceError::Unauthorized
    )]
    pub buyer_nft_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub escrow_nft_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Listing {
    pub seller: Pubkey,
    pub mint:   Pubkey,
    pub price:  u64,
    pub bump:   u8,
}

#[error]
pub enum MarketplaceError {
    #[msg("Unauthorized account.")]
    Unauthorized,
    #[msg("Cannot purchase your own listing.")]
    SelfPurchase,
    #[msg("Insufficient funds.")]
    InsufficientFunds,
    #[msg("Arithmetic overflow.")]
    Overflow,
    #[msg("Escrow account mismatch.")]
    MismatchEscrow,
}
