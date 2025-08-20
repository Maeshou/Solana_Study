use anchor_lang::prelude::*;
use anchor_spl::token::{Transfer, TokenAccount, Token};

declare_id!("L1sTNiFTyXyZaBcDeFgHiJkLmNoPqRsTuVwXyZaBc");

#[program]
pub mod nft_market_listing {
    use super::*;

    /// seller_account から escrow_account へ NFT を移動して出品するが、
    /// 同一アカウントチェックが入っていないため、
    /// seller_account.key() == escrow_account.key() のまま処理が進む可能性がある
    pub fn list_nft(
        ctx: Context<ListNft>,
        price: u64,
    ) -> Result<()> {
        let seller   = &mut ctx.accounts.seller_account;
        let escrow   = &mut ctx.accounts.escrow_account;
        let now      = ctx.accounts.clock.unix_timestamp;

        // NFT を escrow のトークン口座に転送
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.seller_nft_account.to_account_info(),
                to:   ctx.accounts.escrow_nft_account.to_account_info(),
                authority: ctx.accounts.seller.to_account_info(),
            },
        );
        anchor_spl::token::transfer(cpi_ctx, 1)?;

        // 出品件数を増加
        seller.listing_count      = seller.listing_count.wrapping_add(1);
        escrow.locked_nft_count   = escrow.locked_nft_count.wrapping_add(1);

        // 値段を記録
        seller.total_listed_value = seller.total_listed_value + price;
        escrow.total_value_locked = escrow.total_value_locked + price;

        // メモ欄に履歴を追加
        seller.note = format!("{}|listed@{}:{}", seller.note, now, price);
        escrow.note = format!("{}|locked@{}", escrow.note, now);

        msg!(
            "User {} listed NFT at price {} lamports (listing #{})",
            ctx.accounts.seller.key(),
            price,
            seller.listing_count
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ListNft<'info> {
    #[account(mut)]
    pub seller_account:      Account<'info, UserAccount>,
    #[account(mut)]
    pub escrow_account:      Account<'info, EscrowAccount>,
    #[account(mut)]
    pub seller_nft_account:  Account<'info, TokenAccount>,
    #[account(mut)]
    pub escrow_nft_account:  Account<'info, TokenAccount>,
    #[account(signer)]
    pub seller:              Signer<'info>,
    pub token_program:       Program<'info, Token>,
    pub clock:               Sysvar<'info, Clock>,
    pub system_program:      Program<'info, System>,
}

#[account]
pub struct UserAccount {
    pub owner:                Pubkey,
    pub listing_count:        u32,
    pub total_listed_value:   u64,
    pub note:                 String,
}

#[account]
pub struct EscrowAccount {
    pub owner:                Pubkey,
    pub locked_nft_count:     u32,
    pub total_value_locked:   u64,
    pub note:                 String,
}
