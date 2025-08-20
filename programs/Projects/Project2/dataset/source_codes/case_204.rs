use anchor_lang::prelude::*;

declare_id!("Mkt07Game0000000000000000000000000000007");

#[program]
pub mod market_place {
    use super::*;

    pub fn list_item(ctx: Context<ListItem>, nft_id: u64, price: u64) -> Result<()> {
        let l = &mut ctx.accounts.listing;
        l.id = nft_id;
        l.amount = price;
        l.active = true;
        Ok(())
    }

    pub fn delist_item(ctx: Context<ModifyListing>) -> Result<()> {
        let l = &mut ctx.accounts.listing;
        l.active = false;
        Ok(())
    }

    pub fn update_price(ctx: Context<ModifyListing>, new_price: u64) -> Result<()> {
        let l = &mut ctx.accounts.listing;
        if l.active {
            l.amount = new_price;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ListItem<'info> {
    #[account(
        init,
        seeds = [b"listing", user.key().as_ref(), &nft_id.to_le_bytes()],
        bump,
        payer = user,
        space = 8 + 8 + 8 + 1
    )]
    pub listing: Account<'info, ListingData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyListing<'info> {
    #[account(mut, seeds = [b"listing", user.key().as_ref(), &listing.id.to_le_bytes()], bump)]
    pub listing: Account<'info, ListingData>,
    pub user: Signer<'info>,
}

#[account]
pub struct ListingData {
    pub id: u64,
    pub amount: u64,
    pub active: bool,
}
