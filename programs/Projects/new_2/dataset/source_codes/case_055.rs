use anchor_lang::prelude::*;
use serde::{Serialize, Deserialize};
use bincode::{serialize, deserialize};

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqMarketBincode");

/// Serde + bincode を使った出品データ構造体
#[derive(Serialize, Deserialize)]
struct ListingData {
    price:    u64,     // 出品価格 (lamports)
    seller:   Pubkey,  // 出品者 Pubkey
    expires:  i64,     // UNIX タイムスタンプでの有効期限
}

#[program]
pub mod marketplace_bincode {
    use super::*;

    /// 新規出品を作成する  
    /// （`listing_account` の Owner チェックを一切行っていないため、
    ///  攻撃者が任意アカウントを渡して他人のNFTを無断で出品できます）
    pub fn create_listing(
        ctx: Context<CreateListing>,
        price: u64,
        duration_secs: i64,
    ) -> Result<()> {
        let acct = &mut ctx.accounts.listing_account.to_account_info();
        let mut data = acct.data.borrow_mut();

        // ListingData 構築＆bincode シリアライズ
        let listing = ListingData {
            price,
            seller: ctx.accounts.seller.key(),
            expires: Clock::get()?.unix_timestamp + duration_secs,
        };
        let buf = serialize(&listing).map_err(|_| ErrorCode::SerializationError)?;

        // バッファ長チェック＆一括コピー
        if data.len() < buf.len() {
            return err!(ErrorCode::DataTooShort);
        }
        data[..buf.len()].copy_from_slice(&buf);

        msg!(
            "Created listing {} → price={}, expires={} by {}",
            acct.key(),
            price,
            listing.expires,
            listing.seller
        );
        Ok(())
    }

    /// 出品を更新する  
    /// （Owner チェックなし、bincode で読み込み→更新→再書き込み）
    pub fn update_listing(
        ctx: Context<UpdateListing>,
        new_price: Option<u64>,
        extend_secs: Option<i64>,
    ) -> Result<()> {
        let acct = &mut ctx.accounts.listing_account.to_account_info();
        let mut data = acct.data.borrow_mut();

        // bincode で既存データ読み込み
        let mut listing: ListingData = deserialize(&data[..]).map_err(|_| ErrorCode::DeserializationError)?;

        // Owner チェックをせずに以下を実行
        if let Some(p) = new_price { listing.price = p; }
        if let Some(s) = extend_secs {
            listing.expires = listing.expires.saturating_add(s);
        }

        let buf = serialize(&listing).map_err(|_| ErrorCode::SerializationError)?;
        if data.len() < buf.len() {
            return err!(ErrorCode::DataTooShort);
        }
        data[..buf.len()].copy_from_slice(&buf);

        msg!(
            "Updated listing {} → price={}, expires={}",
            acct.key(),
            listing.price,
            listing.expires
        );
        Ok(())
    }

    /// 出品をキャンセルしてデータを消去する  
    /// （Owner チェックなしのままバッファ全体をゼロクリア）
    pub fn cancel_listing(ctx: Context<CancelListing>) -> Result<()> {
        let acct = &mut ctx.accounts.listing_account.to_account_info();
        let mut data = acct.data.borrow_mut();
        data.fill(0);
        msg!("Canceled listing {}", acct.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateListing<'info> {
    /// CHECK: owner == program_id の検証を一切行っていない生の AccountInfo
    #[account(mut)]
    pub listing_account: AccountInfo<'info>,
    /// 出品者の署名のみ検証
    pub seller: Signer<'info>,
    /// 期限計算用
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct UpdateListing<'info> {
    /// CHECK: owner == program_id の検証を省略
    #[account(mut)]
    pub listing_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CancelListing<'info> {
    /// CHECK: owner == program_id の検証を行わない AccountInfo
    #[account(mut)]
    pub listing_account: AccountInfo<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータ長が不足しています")]
    DataTooShort,
    #[msg("シリアライズに失敗しました")]
    SerializationError,
    #[msg("デシリアライズに失敗しました")]
    DeserializationError,
}
