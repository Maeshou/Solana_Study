// =============================================================================
// 4. Marketplace with Owner and Authority Checks
// =============================================================================
#[program]
pub mod secure_marketplace {
    use super::*;

    pub fn list_item(ctx: Context<ListItem>, price: u64, name: String) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        listing.seller = ctx.accounts.seller.key();
        listing.price = price;
        listing.name = name;
        listing.is_active = true;
        listing.bump = *ctx.bumps.get("listing").unwrap();
        Ok(())
    }

    pub fn purchase_item(ctx: Context<PurchaseItem>) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        
        // Transfer lamports from buyer to seller
        let lamports = listing.price;
        **ctx.accounts.buyer.lamports.borrow_mut() -= lamports;
        **ctx.accounts.seller.lamports.borrow_mut() += lamports;
        
        listing.is_active = false;
        Ok(())
    }
}

#[account]
pub struct Listing {
    pub seller: Pubkey,
    pub price: u64,
    pub name: String,
    pub is_active: bool,
    pub bump: u8,
}

#[derive(Accounts)]
#[instruction(price: u64, name: String)]
pub struct ListItem<'info> {
    #[account(
        init,
        payer = seller,
        space = 8 + 32 + 8 + 4 + name.len() + 1 + 1,
        seeds = [b"listing", seller.key().as_ref()],
        bump
    )]
    pub listing: Account<'info, Listing>,
    
    #[account(mut)]
    pub seller: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PurchaseItem<'info> {
    #[account(
        mut,
        seeds = [b"listing", seller.key().as_ref()],
        bump = listing.bump,
        constraint = listing.seller == seller.key(),
        constraint = listing.is_active @ MarketplaceError::ListingNotActive
    )]
    pub listing: Account<'info, Listing>,
    
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    /// CHECK: This account is verified through the listing's seller field
    #[account(mut)]
    pub seller: AccountInfo<'info>,
}

#[error_code]
pub enum MarketplaceError {
    #[msg("Listing is not active")]
    ListingNotActive,
}
