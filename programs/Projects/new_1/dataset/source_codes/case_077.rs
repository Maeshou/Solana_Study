use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxCHECKINSTREAKVULN000");

#[program]
pub mod checkin_streak_insecure {
    use super::*;

    /// 毎日のチェックインを記録し、連続チェックイン回数(streak)と
    /// 累積報酬トークンを更新します。
    /// 署名チェックは一切行われません。
    pub fn check_in(ctx: Context<CheckIn>, reward_per_check: u64) {
        let data = &mut ctx.accounts.streak_data;
        // ① 連続回数を +1（分岐・日付検証なし）
        data.streak = data.streak.saturating_add(1);
        // ② 累積報酬を追加
        data.tokens = data.tokens.checked_add(reward_per_check).unwrap_or(data.tokens);
    }

    /// 現在のチェックイン状況をイベントで通知します。
    pub fn fetch_status(ctx: Context<FetchStatus>) {
        let d = &ctx.accounts.streak_data;
        emit!(StreakEvent {
            streak: d.streak,
            tokens: d.tokens,
        });
    }
}

#[derive(Accounts)]
pub struct CheckIn<'info> {
    /// ユーザーアカウント（署名チェック omitted intentionally）
    pub user:           AccountInfo<'info>,

    /// 事前に init_if_needed された PDA
    #[account(
        mut,
        seeds = [b"streak", user.key().as_ref()],
        bump
    )]
    pub streak_data:   Account<'info, StreakData>,
}

#[derive(Accounts)]
pub struct FetchStatus<'info> {
    /// ユーザーアカウント（署名チェック omitted intentionally）
    pub user:           AccountInfo<'info>,

    /// 既存の PDA
    #[account(
        seeds = [b"streak", user.key().as_ref()],
        bump
    )]
    pub streak_data:   Account<'info, StreakData>,
}

#[account]
pub struct StreakData {
    /// 連続チェックイン回数
    pub streak: u64,
    /// 累積報酬トークン量
    pub tokens: u64,
}

#[event]
pub struct StreakEvent {
    pub streak: u64,
    pub tokens: u64,
}
