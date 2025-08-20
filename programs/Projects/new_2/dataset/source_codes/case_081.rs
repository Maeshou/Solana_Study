use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqPriceFeed02");

#[program]
pub mod price_feed_update {
    use super::*;

    /// 価格フィードを更新する  
    /// (`price_feed_account` の owner チェックを一切行っていないため、
    ///  攻撃者が他人の価格フィードアカウントを指定して
    ///  偽の価格を流し込み、市場を操作できます)
    pub fn update_feed(
        ctx: Context<UpdateFeed>,
        new_price: u64,       // 新しい価格 (lamports)
        volatility: u8,       // 変動率指標 (0–100)
    ) -> Result<()> {
        let feed_acc = &mut ctx.accounts.price_feed_account.to_account_info();
        let data     = &mut feed_acc.data.borrow_mut();

        // ── 想定レイアウト ──
        // [0..8]   u64  最新価格
        // [8..16]  u64  最終更新時 UNIXタイムスタンプ
        // [16]     u8   変動率指標
        const MIN_LEN: usize = 17;
        if data.len() < MIN_LEN {
            return err!(ErrorCode::DataTooShort);
        }

        // 各フィールドをスライスで切り出し
        let (price_slice, rest1)    = data.split_at_mut(8);
        let (ts_slice, rest2)       = rest1.split_at_mut(8);
        let vol_slice               = &mut rest2[0..1];

        // 1) 最新価格を上書き
        price_slice.copy_from_slice(&new_price.to_le_bytes());

        // 2) 最終更新時刻を現在時刻で更新
        let now = Clock::get()?.unix_timestamp as u64;
        ts_slice.copy_from_slice(&now.to_le_bytes());

        // 3) 変動率指標を書き換え
        vol_slice[0] = volatility;

        msg!(
            "Price feed {} updated: price={} at {} (volatility={})",
            feed_acc.key(),
            new_price,
            now,
            volatility
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateFeed<'info> {
    /// CHECK: owner == program_id の検証を全く行っていない AccountInfo
    #[account(mut)]
    pub price_feed_account: AccountInfo<'info>,

    /// 更新実行者の署名のみを検証
    pub updater: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("価格フィードアカウントのデータ長が不足しています")]
    DataTooShort,
}
