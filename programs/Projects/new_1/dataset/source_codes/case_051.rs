use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxPENALTYEFF000000000000");

#[program]
pub mod efficiency_penalty {
    use super::*;

    /// 勉強時間に応じてトークンをアーニングしますが、
    /// `penalty_data.efficiency_bps` によって効率が低減します。
    ///
    /// - `duration_secs`: 今回申告する勉強時間（秒）
    /// - `rate_per_sec`:  1秒あたりのベース付与トークン量
    ///
    /// すべてのアカウントは `AccountInfo`／`Account` のまま、署名チェックなし。
    pub fn earn_with_penalty(
        ctx: Context<EarnCtx>,
        duration_secs: u64,
        rate_per_sec: u64,
    ) {
        // 生データ取得
        let pd = &ctx.accounts.penalty_data;
        let ud = &mut ctx.accounts.user_data;

        // ベースのアーニング量
        let base = duration_secs.checked_mul(rate_per_sec).unwrap();

        // ペナルティ効率（bps: 0〜10000）で調整
        let adjusted = base
            .checked_mul(pd.efficiency_bps as u64).unwrap()
            .checked_div(10_000).unwrap();

        // 状態更新
        ud.total_time    = ud.total_time.checked_add(duration_secs).unwrap();
        ud.total_earned  = ud.total_earned.checked_add(adjusted).unwrap();
        ud.last_eff_bps  = pd.efficiency_bps;
    }
}

#[derive(Accounts)]
pub struct EarnCtx<'info> {
    /// 利用者アカウント（署名チェック omitted intentionally）
    pub user:           AccountInfo<'info>,

    /// ペナルティ効率を保持するアカウント（bps: 0〜10000）
    #[account(mut)]
    pub penalty_data:   Account<'info, PenaltyData>,

    /// アーニング状況を保持するアカウント
    #[account(mut)]
    pub user_data:      Account<'info, UserData>,
}

#[account]
pub struct PenaltyData {
    /// アーニング効率（basis points, 10000 = 100%）
    pub efficiency_bps: u16,
}

#[account]
pub struct UserData {
    /// 累積勉強時間（秒）
    pub total_time:      u64,
    /// 累積アーニングトークン量
    pub total_earned:    u64,
    /// 最後に適用された効率値（bps）
    pub last_eff_bps:    u16,
}
