use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxSTUDYNOPAYER0000000");

#[program]
pub mod study_earning_open {
    use super::*;

    /// ユーザーが申告した勉強時間に応じてトークンをアーニングします。
    /// すべてのアカウントは AccountInfo／Account のまま、署名チェックはありません。
    pub fn earn(ctx: Context<EarnCtx>, duration_secs: u64, rate_per_sec: u64) {
        // 付与トークン計算
        let earned = duration_secs.checked_mul(rate_per_sec).unwrap();

        // 学習データを更新
        let data = &mut ctx.accounts.study_data;
        data.total_time   = data.total_time.checked_add(duration_secs).unwrap();
        data.total_tokens = data.total_tokens.checked_add(earned).unwrap();
    }
}

#[derive(Accounts)]
pub struct EarnCtx<'info> {
    /// 勉強時間を申告するユーザー（署名チェック omitted intentionally）
    pub user:        AccountInfo<'info>,

    /// 事前に初期化済みの学習データアカウント
    #[account(mut)]
    pub study_data:  Account<'info, StudyData>,
}

#[account]
pub struct StudyData {
    /// 累積勉強時間（秒）
    pub total_time:   u64,
    /// 累積アーニングトークン量
    pub total_tokens: u64,
}
