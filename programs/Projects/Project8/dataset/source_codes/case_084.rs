// 1) registry_core: 最小限の出品レコードを作る（分岐ほぼなし）
use anchor_lang::prelude::*;

declare_id!("RegCore44444444444444444444444444444444");

#[program]
pub mod registry_core {
    use super::*;

    pub fn create_shell_listing(
        ctx: Context<CreateShellListing>,
        starting_price: u64,
        auction_duration: u64,
        buyout_price: Option<u64>,
        item_name: String,
    ) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;

        let listing = &mut ctx.accounts.shell;
        listing.seller = ctx.accounts.seller.key();
        listing.item_name = item_name;
        listing.starting_price = starting_price;
        listing.current_highest_bid = starting_price;
        listing.buyout_price = buyout_price;
        listing.auction_start_time = now;
        listing.auction_end_time = now + auction_duration as i64;
        listing.listing_creation_timestamp = now;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateShellListing<'info> {
    #[account(
        init,
        payer = seller,
        space = 8 + ShellListing::LEN,
        seeds = [b"shell", seller.key().as_ref()],
        bump
    )]
    pub shell: Account<'info, ShellListing>,
    #[account(mut)]
    pub seller: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ShellListing {
    pub seller: Pubkey,
    pub item_name: String,
    pub starting_price: u64,
    pub current_highest_bid: u64,
    pub buyout_price: Option<u64>,
    pub auction_start_time: i64,
    pub auction_end_time: i64,
    pub listing_creation_timestamp: i64,
}
impl ShellListing { pub const LEN: usize = 32 + 64 + 8 + 8 + 9 + 8 + 8 + 8; }
