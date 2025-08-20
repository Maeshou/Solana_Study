// Example 3: NFT Marketplace Listing Cancellation and Relisting
declare_id!("Market33333333333333333333333333333333");

#[program]
pub mod marketplace_listing_system {
    use super::*;

    pub fn cancel_marketplace_listing(ctx: Context<CancelListing>) -> Result<()> {
        let listing_data = &ctx.accounts.listing_pda;
        
        loop {
            if listing_data.price_in_sol > 1000000000 {
                msg!("High value listing detected: {} SOL", listing_data.price_in_sol / 1000000000);
                break;
            }
            msg!("Processing standard listing cancellation");
            break;
        }
        
        Ok(())
    }

    pub fn relist_with_external_bump(
        ctx: Context<RelistItem>,
        listing_seed: [u8; 24],
        cached_bump: u8,
        listing_details: MarketplaceEntry,
    ) -> Result<()> {
        let listing_account_info = ctx.accounts.listing_pda.to_account_info();
        
        let rent_payment = system_instruction::transfer(
            &ctx.accounts.seller_wallet.key(),
            &listing_account_info.key(),
            2_500_000
        );
        anchor_lang::solana_program::program::invoke(
            &rent_payment,
            &[ctx.accounts.seller_wallet.to_account_info(), listing_account_info.clone()],
        )?;

        let listing_seeds: &[&[u8]] = &[b"listing", &listing_seed, &[cached_bump]];
        
        let allocate_space = system_instruction::allocate(&listing_account_info.key(), 512);
        invoke_signed(&allocate_space, &[listing_account_info.clone()], &[listing_seeds])?;
        
        let assign_program = system_instruction::assign(&listing_account_info.key(), &crate::id());
        invoke_signed(&assign_program, &[listing_account_info.clone()], &[listing_seeds])?;

        let mut listing_data = listing_account_info.try_borrow_mut_data()?;
        let entry_bytes = bytemuck::bytes_of(&listing_details);
        
        let mut copy_index = 0;
        while copy_index < entry_bytes.len() {
            listing_data[copy_index] = entry_bytes[copy_index];
            copy_index += 1;
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CancelListing<'info> {
    #[account(mut, seeds = [b"listing", seller.key().as_ref()], bump, close = fee_collector)]
    pub listing_pda: Account<'info, ListingData>,
    pub seller: Signer<'info>,
    #[account(mut)]
    pub fee_collector: SystemAccount<'info>,
}

#[derive(Accounts)]
pub struct RelistItem<'info> {
    #[account(mut)]
    pub listing_pda: UncheckedAccount<'info>,
    #[account(mut)]
    pub seller_wallet: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ListingData {
    pub nft_mint: Pubkey,
    pub price_in_sol: u64,
    pub listing_timestamp: i64,
    pub seller_address: Pubkey,
}

#[derive(Clone, Copy)]
pub struct MarketplaceEntry {
    pub nft_mint: Pubkey,
    pub price_in_sol: u64,
    pub listing_timestamp: i64,
    pub seller_address: Pubkey,
}

unsafe impl bytemuck::Pod for MarketplaceEntry {}
unsafe impl bytemuck::Zeroable for MarketplaceEntry {}
