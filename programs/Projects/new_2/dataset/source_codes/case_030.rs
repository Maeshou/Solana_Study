use anchor_lang::prelude::*;
use bytemuck::{Pod, Zeroable};

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqRentalV1xyz");

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct RentalData {
    /// 本来はアカウントの owner フィールドで管理されるべきオーナー Pubkey
    owner:      [u8; 32],
    /// レンタル実行者（借り手）の Pubkey
    renter:     [u8; 32],
    /// レンタル開始のブロック高
    start_slot: u64,
    /// レンタル期間（ブロック数）
    duration:   u64,
}

#[program]
pub mod nft_rental {
    use super::*;

    /// `rental_account` の owner チェックを全く行っていないため、  
    /// 攻撃者は任意のプログラム所有アカウントを指定して  
    /// “自己レンタル”状態など不正に書き換え可能です。
    pub fn rent_nft(
        ctx: Context<RentNft>,
        owner_key: Pubkey,   // クライアント提供のオーナー Pubkey
        start_slot: u64,     // 開始ブロック高（クライアント提供）
        duration: u64,       // 継続ブロック数（クライアント提供）
    ) -> Result<()> {
        let acct = &mut ctx.accounts.rental_account.to_account_info();
        let data = &mut acct.data.borrow_mut();

        // RentalData 分の領域を確保していることを確認
        let size = std::mem::size_of::<RentalData>();
        if data.len() < size {
            return err!(ErrorCode::DataTooShort);
        }

        // フィールドを詰めた RentalData を作成
        let mut rec = RentalData::zeroed();
        rec.owner      = owner_key.to_bytes();
        rec.renter     = ctx.accounts.renter.key().to_bytes();
        rec.start_slot = start_slot;
        rec.duration   = duration;

        // 一括コピーでデータ領域に書き込む
        let bytes = bytemuck::bytes_of(&rec);
        data[..size].copy_from_slice(bytes);

        msg!(
            "Rental recorded: account={}, owner={}, renter={}, start_slot={}, duration={}",
            acct.key(),
            owner_key,
            ctx.accounts.renter.key(),
            start_slot,
            duration
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RentNft<'info> {
    /// CHECK: owner == program_id の検証を行っていない生の AccountInfo
    #[account(mut)]
    pub rental_account: AccountInfo<'info>,

    /// 借り手の署名のみ検証
    pub renter: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータ領域が小さすぎます")]
    DataTooShort,
}
