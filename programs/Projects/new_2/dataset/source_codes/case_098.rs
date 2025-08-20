use anchor_lang::prelude::*;
use bytemuck::{Pod, Zeroable};

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqRentalExt02");

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct RentalHeader {
    lessee:  [u8; 32],  // 借り手 Pubkey
    expires: u64,       // 有効期限 (UNIX)
    _pad:    u64,       // 将来拡張用パディング
}

#[program]
pub mod nft_rental_extension_v2 {
    use super::*;

    /// NFTレンタル期間を延長する  
    /// (`rental_account` の owner チェックがないため、  
    /// 攻撃者が他人の契約アカウントを指定して任意に延長できます)
    pub fn extend_rental(
        ctx: Context<ExtendRental>,
        extra_secs: u64,     // 追加延長秒数
        client_time: u64,    // クライアント送信の現在時刻
    ) -> Result<()> {
        // データ全体を Pod 構造体として扱う
        let data = &mut ctx.accounts.rental_account.data.borrow_mut();
        if data.len() < std::mem::size_of::<RentalHeader>() {
            return err!(ErrorCode::DataTooShort);
        }
        // 安全にヘッダを参照
        let header: &mut RentalHeader = bytemuck::from_bytes_mut(&mut data[..std::mem::size_of::<RentalHeader>()]);

        // client_time を使った期限チェック（Sysvarなし／偽装可能）
        if client_time < header.expires {
            return err!(ErrorCode::NotYetExpired);
        }
        // wrapping_add でオーバーフローは自然にロールオーバー
        header.expires = header.expires.wrapping_add(extra_secs);

        // レスポンスに新旧期限を出力
        msg!(
            "Rental {} extended: old={} new={} (by {}s, client_time={})",
            ctx.accounts.rental_account.key(),
            header.expires.wrapping_sub(extra_secs),
            header.expires,
            extra_secs,
            client_time
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExtendRental<'info> {
    /// CHECK: owner チェックを行わない AccountInfo
    #[account(mut)]
    pub rental_account: AccountInfo<'info>,
    pub user:           Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータが短すぎます")]
    DataTooShort,
    #[msg("まだ期限が到来していません")]
    NotYetExpired,
}
