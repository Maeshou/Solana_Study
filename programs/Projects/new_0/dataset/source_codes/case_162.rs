use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzUI");

#[program]
pub mod habit_tracker {
    use super::*;

    /// 習慣作成：ID・説明を受け取り、最終達成時刻を 0、active を true で初期化
    pub fn create_habit(
        ctx: Context<CreateHabit>,
        bump: u8,
        habit_id: u64,
        description: String,
    ) -> Result<()> {
        *ctx.accounts.habit = Habit {
            owner:             ctx.accounts.user.key(),
            bump,
            habit_id,
            description,
            last_completed_ts: 0,
            active:            true,
        };
        Ok(())
    }

    /// 習慣達成登録：last_completed_ts に現在のタイムスタンプを設定
    pub fn complete_habit(
        ctx: Context<ModifyHabit>,
    ) -> Result<()> {
        let h = &mut ctx.accounts.habit;
        h.last_completed_ts = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }

    /// 習慣の有効／無効切替：active フラグをトグル
    pub fn toggle_active(
        ctx: Context<ModifyHabit>,
    ) -> Result<()> {
        let h = &mut ctx.accounts.habit;
        h.active = !h.active;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8, habit_id: u64)]
pub struct CreateHabit<'info> {
    /// PDA で生成する Habit アカウント
    #[account(
        init,
        payer = user,
        // discriminator(8) + owner(32) + bump(1) + habit_id(8)
        // + 4 + description 最大200バイト + 8(last_completed_ts) + 1(active)
        space = 8 + 32 + 1 + 8 + 4 + 200 + 8 + 1,
        seeds = [b"habit", user.key().as_ref(), &habit_id.to_le_bytes()],
        bump
    )]
    pub habit: Account<'info, Habit>,

    /// 習慣所有者（署名必須）
    #[account(mut)]
    pub user: Signer<'info>,

    /// システムプログラム
    pub system_program: Program<'info, System>,

    /// タイムスタンプ取得用
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct ModifyHabit<'info> {
    /// 既存の Habit（PDA 検証 + オーナーチェック）
    #[account(
        mut,
        seeds = [b"habit", owner.key().as_ref(), &habit.habit_id.to_le_bytes()],
        bump = habit.bump,
        has_one = owner
    )]
    pub habit: Account<'info, Habit>,

    /// 習慣所有者（署名必須）
    #[account(signer)]
    pub owner: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    /// タイムスタンプ取得用
    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct Habit {
    pub owner:             Pubkey,
    pub bump:              u8,
    pub habit_id:          u64,
    pub description:       String,
    pub last_completed_ts: i64,
    pub active:            bool,
}
