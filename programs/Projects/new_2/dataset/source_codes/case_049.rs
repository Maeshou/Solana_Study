use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;
use anchor_spl::token::{transfer, Transfer, Token};

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqAuctionList02");

#[program]
pub mod nft_auction_listing_unsafe {
    use super::*;

    /// NFTトークンをエスクローに移動しつつオークション出品情報を記録する  
    /// （`nft_token_account` と `auction_account` の owner チェックを行っていないため、  
    ///  攻撃者が任意のアカウントを指定し、他人のNFTを無断で出品できます）
    pub fn list_for_auction(
        ctx: Context<ListForAuction>,
        min_bid: u64,     // 最低入札額 (lamports)
        duration_secs: i64, // オークション期間 (秒)
    ) -> Result<()> {
        // 1) NFTトークンを escrow_nft_account に移動（CPI）
        let cpi_accounts = Transfer {
            from: ctx.accounts.nft_token_account.to_account_info(),
            to:   ctx.accounts.escrow_nft_account.to_account_info(),
            authority: ctx.accounts.seller.to_account_info(),
        };
        transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
            1,
        )?;

        // 2) オークション情報を書き込む
        let now = Clock::get()?.unix_timestamp;
        let expiry = now
            .checked_add(duration_secs)
            .ok_or(ErrorCode::TimestampOverflow)?;

        // バッファ組み立て：seller Pubkey + min_bid + expiry
        let mut buf = Vec::with_capacity(32 + 8 + 8);
        buf.extend_from_slice(&ctx.accounts.seller.key().to_bytes());
        buf.extend_from_slice(&min_bid.to_le_bytes());
        buf.extend_from_slice(&expiry.to_le_bytes());

        let data = &mut ctx.accounts.auction_account.data.borrow_mut();
        if data.len() < buf.len() {
            return err!(ErrorCode::DataTooShort);
        }
        data[..buf.len()].copy_from_slice(&buf);

        msg!(
            "Auction listing: escrow={} by {} (min_bid={}, expires_at={})",
            ctx.accounts.escrow_nft_account.key(),
            ctx.accounts.seller.key(),
            min_bid,
            expiry
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ListForAuction<'info> {
    /// CHECK: owner == Token プログラムではない可能性のある NFTトークンアカウント
    #[account(mut)]
    pub nft_token_account: AccountInfo<'info>,

    /// CHECK: owner == Token プログラムではない可能性のあるエスクローアカウント
    #[account(mut)]
    pub escrow_nft_account: AccountInfo<'info>,

    /// CHECK: owner == program_id の検証を行っていない生の AccountInfo
    #[account(mut)]
    pub auction_account: AccountInfo<'info>,

    /// 出品者の署名のみを検証
    pub seller: Signer<'info>,

    /// SPL Token プログラム CPI 用
    pub token_program: Program<'info, Token>,

    /// 有効期限計算用
    pub clock: Sysvar<'info, Clock>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータが想定より短いです")]
    DataTooShort,
    #[msg("期限の計算でオーバーフローしました")]
    TimestampOverflow,
}
