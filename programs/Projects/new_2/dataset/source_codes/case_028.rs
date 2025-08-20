use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqCancelV2Long");

#[program]
pub mod nft_cancel_listing_v2 {
    use super::*;

    /// マーケットプレイス出品をキャンセルし、  
    /// 出品情報の一部をクリア／更新する  
    /// （owner チェックを行っていないため、任意のアカウントで操作可能）
    pub fn cancel_listing(
        ctx: Context<CancelListing>,
    ) -> Result<()> {
        let listing_acc = &mut ctx.accounts.listing_account.to_account_info();
        let mut data = listing_acc.data.borrow_mut();

        // ── バイトレイアウト想定 ──
        // [0..8]   u64  price
        // [8..10]  u16  listing_fee_bps
        // [10..18] u64  expiry_slot
        // [18..50] [u8;32] seller Pubkey
        // [50..58] u64  created_timestamp
        // [58..62] u32  cancel_count

        const REQUIRED: usize = 62;
        if data.len() < REQUIRED {
            return err!(ErrorCode::DataTooShort);
        }

        // フィールドごとにスライスを分割
        let (price_slice, rest1)       = data.split_at_mut(8);
        let (_fee_slice, rest2)        = rest1.split_at_mut(2);
        let (expiry_slice, rest3)      = rest2.split_at_mut(8);
        let (_seller_slice, rest4)     = rest3.split_at_mut(32);
        let (created_slice, rest5)     = rest4.split_at_mut(8);
        let (cancel_cnt_slice, _)      = rest5.split_at_mut(4);

        // 1) キャンセル回数をインクリメント
        let prev = u32::from_le_bytes(cancel_cnt_slice.try_into().unwrap());
        let new_cnt = prev.saturating_add(1);
        cancel_cnt_slice.copy_from_slice(&new_cnt.to_le_bytes());

        // 2) 出品価格を 0 にリセット
        price_slice.copy_from_slice(&0u64.to_le_bytes());

        // 3) 有効期限スロットも 0 にクリア
        expiry_slice.copy_from_slice(&0u64.to_le_bytes());

        // 4) キャンセル時刻を created_timestamp フィールドに上書き
        let now = Clock::get()?.unix_timestamp as u64;
        created_slice.copy_from_slice(&now.to_le_bytes());

        msg!(
            "Listing {} canceled by {}. Total cancels: {}",
            listing_acc.key(),
            ctx.accounts.user.key(),
            new_cnt
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CancelListing<'info> {
    /// CHECK: owner == program_id の確認を一切行っていない
    #[account(mut)]
    pub listing_account: AccountInfo<'info>,

    /// 呼び出し元ユーザーの署名のみ検証
    pub user: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータが想定より少ないです")]
    DataTooShort,
}
