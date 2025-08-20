use anchor_lang::prelude::*;

// ── アカウントデータはファイル冒頭にタプル構造体で定義 ──
#[account]
#[derive(Default)]
pub struct HabitTracker(pub u8, pub Vec<(Vec<u8>, u64)>); // (bump, Vec<(habit_name, streak)>)

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzVK");

#[error_code]
pub enum ErrorCode {
    #[msg("Maximum number of habits reached")]
    MaxHabitsReached,
    #[msg("Habit not found")]
    HabitNotFound,
}

#[program]
pub mod habit_tracker {
    use super::*;

    const MAX_HABITS: usize = 12;

    /// 初期化：内部 Vec は空、bump のみ設定
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let b = *ctx.bumps.get("tracker").unwrap();
        ctx.accounts.tracker.0 = b;
        Ok(())
    }

    /// 習慣追加：件数制限チェック＋初期ストリーク 0 で追加
    pub fn add_habit(ctx: Context<Modify>, name: Vec<u8>) -> Result<()> {
        let list = &mut ctx.accounts.tracker.1;
        if list.len() >= MAX_HABITS {
            return err!(ErrorCode::MaxHabitsReached);
        }
        list.push((name, 0));
        Ok(())
    }

    /// ストリーク更新：該当習慣を検索し、+1
    pub fn increment_streak(ctx: Context<Modify>, name: Vec<u8>) -> Result<()> {
        let list = &mut ctx.accounts.tracker.1;
        let mut found = false;
        for entry in list.iter_mut() {
            if entry.0 == name {
                entry.1 = entry.1.wrapping_add(1);
                found = true;
            }
        }
        if !found {
            return err!(ErrorCode::HabitNotFound);
        }
        Ok(())
    }

    /// 習慣削除：該当習慣を一括除去
    pub fn remove_habit(ctx: Context<Modify>, name: Vec<u8>) -> Result<()> {
        let list = &mut ctx.accounts.tracker.1;
        list.retain(|(n, _)| {
            if *n == name {
                false
            } else {
                true
            }
        });
        Ok(())
    }

    /// 全ストリークをリセット：すべてのカウントを0に
    pub fn reset_all(ctx: Context<Modify>) -> Result<()> {
        let list = &mut ctx.accounts.tracker.1;
        for entry in list.iter_mut() {
            entry.1 = 0;
        }
        Ok(())
    }

    /// 現在の習慣数をログ出力
    pub fn count_habits(ctx: Context<Modify>) -> Result<()> {
        let cnt = ctx.accounts.tracker.1.len() as u64;
        msg!("Total habits tracked: {}", cnt);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_zeroed,
        payer = authority,
        seeds = [b"tracker", authority.key().as_ref()],
        bump,
        // discriminator(8) + bump(1) + Vec len(4) + max12*(4+32+8)
        // habit_name: max 32-byte UTF-8 name
        space = 8 + 1 + 4 + 12 * (4 + 32 + 8)
    )]
    pub tracker:   Account<'info, HabitTracker>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Modify<'info> {
    #[account(
        mut,
        seeds = [b"tracker", authority.key().as_ref()],
        bump = tracker.0,
    )]
    pub tracker:   Account<'info, HabitTracker>,
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
