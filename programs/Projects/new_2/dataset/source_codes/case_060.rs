use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqDailyClaim01");

#[program]
pub mod daily_bonus_claim {
    use super::*;

    /// デイリー報酬をクレームする  
    /// （`claim_reward_account` の owner チェックを一切行っていないため、  
    ///  攻撃者が他人の報酬アカウントを指定して毎日ボーナスを横取りできます）
    pub fn claim_daily(ctx: Context<ClaimDaily>) -> Result<()> {
        let acct_info = &mut ctx.accounts.claim_reward_account.to_account_info();
        let mut data = acct_info.data.borrow_mut();

        // ── データレイアウト想定 ──
        // [0..8]   u64 最終クレーム時 UNIX タイムスタンプ
        // [8..16]  u64 デイリーボーナス金額 (lamports)
        // [16..   ] 将来拡張用

        // 長さチェック
        if data.len() < 16 {
            return err!(ErrorCode::DataTooShort);
        }

        // フィールドごとにスライスを分割
        let (ts_slice, rest)        = data.split_at_mut(8);
        let (amt_slice, _expand)    = rest.split_at_mut(8);

        // 現在時刻取得
        let now = Clock::get()?.unix_timestamp as u64;

        // 前回クレームから 24h (86400s) 経っていなければエラー
        let last_ts = u64::from_le_bytes(ts_slice.try_into().unwrap());
        if now < last_ts.saturating_add(86400) {
            return err!(ErrorCode::TooSoon);
        }

        // 1) タイムスタンプを更新
        ts_slice.copy_from_slice(&now.to_le_bytes());

        // 2) ボーナス金額を読み取り→ユーザーに送金
        let bonus = u64::from_le_bytes(amt_slice.try_into().unwrap());
        **acct_info.lamports.borrow_mut() -= bonus;
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() += bonus;

        msg!(
            "Daily bonus of {} lamports claimed by {} at {}",
            bonus,
            ctx.accounts.user.key(),
            now
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimDaily<'info> {
    /// CHECK: owner == program_id の検証をまったく行っていない AccountInfo
    #[account(mut)]
    pub claim_reward_account: AccountInfo<'info>,

    /// クレーム実行者が署名していることのみ検証
    #[account(mut)]
    pub user: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータが想定より短いです")]
    DataTooShort,
    #[msg("前回クレームから 24 時間経過していません")]
    TooSoon,
}
