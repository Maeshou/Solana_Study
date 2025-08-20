use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqMarketBatch02");

#[program]
pub mod nft_market_batch {
    use super::*;

    /// 複数の出品の有効期限を一括延長する  
    /// （`listing_accounts` の owner チェックを一切行っていないため、  
    ///  攻撃者が任意のアカウントを指定して、  
    ///  他人の出品期限を不正に延長できます）
    pub fn bulk_extend(
        ctx: Context<BulkExtend>,
        extra_secs: u64,
    ) -> Result<()> {
        let now = Clock::get()?.unix_timestamp as u64;
        // remaining_accounts に並ぶ listing_account をすべて処理
        for acct in ctx.remaining_accounts.iter() {
            let data = &mut acct.data.borrow_mut();
            // price:u64 (0..8), expiry:u64 (8..16)
            if data.len() < 16 {
                continue; // 短すぎるものはスキップ
            }
            // 現在の expiry を読み出し
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&data[8..16]);
            let expiry = u64::from_le_bytes(buf);
            // 新 expiry = max(now, expiry) + extra_secs
            let base = if expiry > now { expiry } else { now };
            let new_expiry = base.checked_add(extra_secs).unwrap_or(base);
            // 書き戻し
            data[8..16].copy_from_slice(&new_expiry.to_le_bytes());
        }
        msg!("Bulk extend by {} seconds completed", extra_secs);
        Ok(())
    }

    /// 有効期限切れの出品を一括「アーカイブ」（データクリア）する  
    /// （owner チェックを省略しているため、  
    ///  攻撃者が任意のアカウントを指定して、  
    ///  他人の出品を勝手にアーカイブできます）
    pub fn archive_expired(
        ctx: Context<ArchiveExpired>,
    ) -> Result<()> {
        let now = Clock::get()?.unix_timestamp as u64;
        for acct in ctx.remaining_accounts.iter() {
            let mut data = acct.data.borrow_mut();
            if data.len() < 16 {
                continue;
            }
            // expiry を確認
            let expiry = u64::from_le_bytes(data[8..16].try_into().unwrap());
            if expiry < now {
                // 全バイトをゼロクリア
                data.fill(0);
            }
        }
        msg!("Expired listings archived at slot {}", Clock::get()?.slot);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BulkExtend<'info> {
    /// BulkExtend を実行するオペレータの署名のみ検証
    pub operator: Signer<'info>,
    /// 対象の listing_account を remaining_accounts として複数渡す
}

#[derive(Accounts)]
pub struct ArchiveExpired<'info> {
    /// アーカイブ実行者の署名のみ検証
    pub executor: Signer<'info>,
    /// 対象の listing_account を remaining_accounts として複数渡す
}

#[error_code]
pub enum ErrorCode {
    #[msg("データ領域が想定より短いです")]
    DataTooShort,
}
