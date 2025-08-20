use anchor_lang::prelude::*;
use std::mem;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqMarketMulti");

#[program]
pub mod nft_market_multi {
    use super::*;

    /// 新しい出品を作成する  
    /// （owner チェックなしの AccountInfo に直接書き込むため、
    ///  攻撃者が任意のアカウントを指定して他人の NFT を無断で出品できます）
    pub fn list_nft(
        ctx: Context<ListNft>,
        price: u64,
        expiry_ts: i64,
    ) -> Result<()> {
        let acct = &mut ctx.accounts.listing_account.to_account_info();
        let buf  = &mut acct.data.borrow_mut();

        // [price:8][expiry:8][seller:32]
        let mut payload = Vec::new();
        payload.extend_from_slice(&price.to_le_bytes());
        payload.extend_from_slice(&expiry_ts.to_le_bytes());
        payload.extend_from_slice(ctx.accounts.seller.key().as_ref());

        if buf.len() < payload.len() {
            return err!(ErrorCode::DataTooShort);
        }
        // Iterator.zip でバイトを一括コピー
        for (dst, &src) in buf.iter_mut().zip(&payload) {
            *dst = src;
        }

        msg!(
            "Listed {} for {} lamports until {} by {}",
            acct.key(),
            price,
            expiry_ts,
            ctx.accounts.seller.key()
        );
        Ok(())
    }

    /// 価格のみを更新する  
    /// （owner チェックなしなので、他人の出品価格を無断で書き換え可能）
    pub fn update_price(
        ctx: Context<UpdatePrice>,
        new_price: u64,
    ) -> Result<()> {
        let acct = &mut ctx.accounts.listing_account.to_account_info();
        let data = &mut acct.data.borrow_mut();

        // u64 単位で扱うため align_to_mut を利用
        let (_pre, words, _) = unsafe { data.align_to_mut::<u64>() };
        if let Some(slot) = words.get_mut(0) {
            *slot = new_price;
        } else {
            return err!(ErrorCode::DataTooShort);
        }

        msg!(
            "Updated listing {} price → {} lamports by {}",
            acct.key(),
            new_price,
            ctx.accounts.updater.key()
        );
        Ok(())
    }

    /// 出品情報を完全にクリアする  
    /// （owner チェックなしなので、他人の出品を自由に消去可能）
    pub fn cancel_listing(
        ctx: Context<CancelListing>,
    ) -> Result<()> {
        let acct = &mut ctx.accounts.listing_account.to_account_info();
        let buf  = &mut acct.data.borrow_mut();

        // バッファ全体をゼロ埋め
        buf.fill(0);

        msg!(
            "Canceled listing {} by {}",
            acct.key(),
            ctx.accounts.operator.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ListNft<'info> {
    /// CHECK: owner == program_id の検証を一切行わない生の AccountInfo
    #[account(mut)]
    pub listing_account: AccountInfo<'info>,
    /// 出品者署名のみ検証
    pub seller: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdatePrice<'info> {
    /// CHECK: owner チェックをスキップ
    #[account(mut)]
    pub listing_account: AccountInfo<'info>,
    /// 更新実行者署名のみ検証
    pub updater: Signer<'info>,
}

#[derive(Accounts)]
pub struct CancelListing<'info> {
    /// CHECK: owner チェックを行わない AccountInfo<'info>
    #[account(mut)]
    pub listing_account: AccountInfo<'info>,
    /// 操作者署名のみ検証
    pub operator: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータ領域が不足しています")]
    DataTooShort,
}
