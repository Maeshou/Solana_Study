use anchor_lang::prelude::*;
use bytemuck::{Pod, Zeroable};

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqGiftNFT01");

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct GiftData {
    nft_key:      [u8; 32],  // 贈与対象 NFT の Pubkey
    sender_key:   [u8; 32],  // 送信者の Pubkey
    recipient_key:[u8; 32],  // 受信者の Pubkey
    timestamp:    u64,       // 贈与時 UNIX タイムスタンプ
}

#[program]
pub mod nft_gifting {
    use super::*;

    /// `gift_account` の owner チェックを行っていないため、
    /// 攻撃者が任意のアカウントを指定し、他人の NFT を
    /// 自分宛に「贈与」したことにできます。
    pub fn gift_nft(
        ctx: Context<GiftNft>,
    ) -> Result<()> {
        let acct = &mut ctx.accounts.gift_account.to_account_info();
        let data = &mut acct.data.borrow_mut();

        // GiftData 構造体分の領域を確保
        let size = std::mem::size_of::<GiftData>();
        if data.len() < size {
            return err!(ErrorCode::DataTooShort);
        }

        // 現在時刻を取得
        let now_ts = Clock::get()?.unix_timestamp as u64;

        // ギフト情報を構造体に詰める
        let mut gift = GiftData::zeroed();
        gift.nft_key       = ctx.accounts.nft_mint.key().to_bytes();
        gift.sender_key    = ctx.accounts.sender.key().to_bytes();
        gift.recipient_key = ctx.accounts.recipient.key().to_bytes();
        gift.timestamp     = now_ts;

        // 一括コピーでバイトを書き込む
        let bytes = bytemuck::bytes_of(&gift);
        data[..size].copy_from_slice(bytes);

        msg!(
            "NFT {} gifted from {} to {} at {}",
            ctx.accounts.nft_mint.key(),
            ctx.accounts.sender.key(),
            ctx.accounts.recipient.key(),
            now_ts
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct GiftNft<'info> {
    /// CHECK: owner フィールドの検証を全く行っていない AccountInfo
    #[account(mut)]
    pub gift_account: AccountInfo<'info>,

    /// 贈与対象 NFT の Mint アカウント（所有者チェックなし）
    pub nft_mint: AccountInfo<'info>,

    /// 贈与者（署名のみ検証）
    pub sender: Signer<'info>,

    /// 受贈者アカウント（受取先 Pubkey を記録するのみ）
    pub recipient: AccountInfo<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("gift_account のデータ領域が不足しています")]
    DataTooShort,
}
