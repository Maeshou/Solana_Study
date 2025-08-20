use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("HabT111111111111111111111111111111111111");

#[program]
pub mod habit_tracker {
    /// 習慣を新規作成
    pub fn create_habit(ctx: Context<CreateHabit>, name: String) -> Result<()> {
        // 名前長チェック
        if name.len() > 32 {
            return Err(ErrorCode::NameTooLong.into());
        }
        let habit = &mut ctx.accounts.habit;
        habit.owner     = ctx.accounts.user.key();
        habit.name      = name;
        habit.count     = 0;
        habit.last_done = 0;
        Ok(())
    }

    /// 今日の習慣を完了マーク
    pub fn mark_done(ctx: Context<MarkDone>) -> Result<()> {
        let habit = &mut ctx.accounts.habit;
        let user = ctx.accounts.user.key();

        // 所有者チェック
        if habit.owner != user {
            return Err(ErrorCode::Unauthorized.into());
        }

        // 日付重複チェック
        let now = ctx.accounts.clock.unix_timestamp;
        if now <= habit.last_done {
            return Err(ErrorCode::AlreadyDone.into());
        }

        habit.last_done = now;
        habit.count = habit
            .count
            .checked_add(1)
            .ok_or(ErrorCode::Overflow)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateHabit<'info> {
    /// init で再初期化防止
    #[account(init, payer = user, space = 8 + 32 + 4 + 32 + 8 + 8)]
    pub habit:          Account<'info, HabitAccount>,

    /// 習慣作成者（署名必須）
    #[account(mut)]
    pub user:           Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MarkDone<'info> {
    /// Account<> による Owner Check & Type Cosplay
    #[account(mut)]
    pub habit: Account<'info, HabitAccount>,

    /// 習慣を完了マークするユーザー（署名必須）
    pub user:  Signer<'info>,

    /// 現在時刻取得用
    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct HabitAccount {
    /// この習慣を操作できるユーザー
    pub owner:     Pubkey,
    /// 習慣名（最大32文字）
    pub name:      String,
    /// 完了日数カウント
    pub count:     u64,
    /// 最後に完了マークした UNIX タイムスタンプ
    pub last_done: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("権限がありません")]
    Unauthorized,
    #[msg("名前が長すぎます")]
    NameTooLong,
    #[msg("本日はすでに完了マーク済みです")]
    AlreadyDone,
    #[msg("カウントがオーバーフローしました")]
    Overflow,
}
