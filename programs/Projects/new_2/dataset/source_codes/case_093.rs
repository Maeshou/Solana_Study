use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;
use bytemuck::{Pod, Zeroable};

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqItemEnh03");

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct EnhanceRecord {
    level:         u8,     // 現在の強化レベル
    enhancements:  u8,     // 累計強化回数
    reserved:      [u8;6], // パディング
    timestamp:     u64,    // 最終強化時刻
    // user_pubkey follows immediately in the buffer, but not part of this struct
}

#[program]
pub mod nft_item_enhance_v3 {
    use super::*;

    /// アイテムを強化し、強化回数とタイムスタンプを更新する  
    /// (`item_account` の owner チェックを一切行っていないため、  
    ///  他人のアイテムを好きなだけ強化できます)
    pub fn enhance_item(
        ctx: Context<EnhanceItem>,
        inc: u8,   // 増加させる強化レベル数
    ) -> Result<()> {
        let now = ctx.accounts.clock.unix_timestamp as u64;
        let data = &mut ctx.accounts.item_account.data.borrow_mut();

        // バッファが EnhanceRecord + Pubkey (32) の最小長を持つかチェック
        let min_len = std::mem::size_of::<EnhanceRecord>() + 32;
        if data.len() < min_len {
            return err!(ErrorCode::DataTooShort);
        }

        // 最初の EnhanceRecord 部分を安全に参照
        let (head, tail) = data.split_at_mut(std::mem::size_of::<EnhanceRecord>());
        let mut record: &mut EnhanceRecord = bytemuck::from_bytes_mut(head);

        // saturating_add でレベル＆累計回数を更新、オーバーフロー防止
        record.level        = record.level.saturating_add(inc);
        record.enhancements = record.enhancements.saturating_add(inc);

        // timestamp 更新
        record.timestamp = now;

        // 残りの 32 バイトに操作ユーザー Pubkey を直接書き込む
        let user_pk = ctx.accounts.user.key().to_bytes();
        tail[..32].copy_from_slice(&user_pk);

        msg!(
            "Enhanced {}: level→{} (＋{}), total→{} at {} by {}",
            ctx.accounts.item_account.key(),
            record.level,
            inc,
            record.enhancements,
            now,
            ctx.accounts.user.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct EnhanceItem<'info> {
    /// CHECK: owner == program_id の検証を省略している AccountInfo
    #[account(mut)]
    pub item_account: AccountInfo<'info>,

    /// Clock Sysvar を受け取る
    pub clock: Sysvar<'info, Clock>,

    /// 操作ユーザーの署名のみ検証
    pub user: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アイテムアカウントのデータ長が不足しています")]
    DataTooShort,
}
