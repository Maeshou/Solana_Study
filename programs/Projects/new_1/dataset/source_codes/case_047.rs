use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxSTUDYREWARD000000000000");

#[program]
pub mod study_earning {
    use super::*;

    /// 勉強時間に応じてトークンをアーニングします。
    /// - `duration_secs`: 今回報告する勉強時間（秒単位）
    /// - `rate_per_sec` : 1秒あたりのトークン付与量
    /// すべてのアカウントは AccountInfo／Account のまま、署名チェックは行いません。
    pub fn earn(ctx: Context<EarnCtx>, duration_secs: u64, rate_per_sec: u64) {
        // 付与トークン計算
        let earned = duration_secs.checked_mul(rate_per_sec).unwrap();
        // PDA に累積勉強時間とトークン残高を更新
        let data = &mut ctx.accounts.study_data;
        data.total_time   = data.total_time.checked_add(duration_secs).unwrap();
        data.total_earned = data.total_earned.checked_add(earned).unwrap();
    }
}

#[derive(Accounts)]
pub struct EarnCtx<'info> {
    /// トランザクション手数料支払い用（署名必須）
    #[account(mut)]
    pub fee_payer:   Signer<'info>,

    /// 勉強時間を申告するユーザー（署名チェック omitted intentionally）
    pub user:        AccountInfo<'info>,

    /// ユーザーごとの勉強・報酬データを保持する PDA
    #[account(
        init_if_needed,
        payer     = fee_payer,
        seeds     = [b"study", user.key().as_ref()],
        bump,
        space     = 8 + 8 + 8
    )]
    pub study_data:  Account<'info, StudyData>,

    pub system_program: Program<'info, System>,
    pub rent:           Sysvar<'info, Rent>,
}

#[account]
pub struct StudyData {
    /// 累積勉強時間（秒）
    pub total_time:    u64,
    /// 累積アーニングトークン量
    pub total_earned:  u64,
}
