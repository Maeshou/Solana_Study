use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("MARKET11111111111111111111111111111111111111");

#[program]
pub mod marketplace_program {
    use super::*;
    /// NFTの所有権をプログラムに移管し、価格を設定して出品リストを作成します。
    pub fn list_nft_for_sale(ctx: Context<ListNftOnMarketplace>, price_lamports: u64) -> Result<()> {
        let listing = &mut ctx.accounts.item_listing;
        listing.seller = *ctx.accounts.seller.key;
        listing.item_mint = ctx.accounts.item_mint.key();
        listing.price = price_lamports;
        listing.is_active = true;
        
        let cpi_accounts = Transfer {
            from: ctx.accounts.seller_item_token_account.to_account_info(),
            to: ctx.accounts.vault_item_token_account.to_account_info(),
            authority: ctx.accounts.seller.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_context, 1)?;

        msg!("NFT {} listed for sale", listing.item_mint);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ListNftOnMarketplace<'info> {
    #[account(init, payer = seller, space = 8 + 32 + 32 + 8 + 1)]
    pub item_listing: Account<'info, ItemListing>,
    #[account(mut)]
    pub seller: Signer<'info>,
    pub item_mint: Account<'info, Mint>,
    #[account(mut, constraint = seller_item_token_account.mint == item_mint.key() && seller_item_token_account.owner == seller.key() && seller_item_token_account.amount == 1)]
    pub seller_item_token_account: Account<'info, TokenAccount>,
    #[account(init, payer = seller, token::mint = item_mint, token::authority = item_listing)]
    pub vault_item_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct ItemListing {
    pub seller: Pubkey,
    pub item_mint: Pubkey,
    pub price: u64,
    pub is_active: bool,
}