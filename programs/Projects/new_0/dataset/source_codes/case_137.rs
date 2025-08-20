use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("Remn111111111111111111111111111111111111");

const MAX_REMINDERS: usize = 50;

#[program]
pub mod reminder_manager {
    /// リマインダーリストの初期化
    pub fn create_list(ctx: Context<CreateList>) -> Result<()> {
        let list = &mut ctx.accounts.reminder_list;
        list.owner     = ctx.accounts.user.key();  // Signer Authorization
        list.reminders = Vec::new();
        Ok(())
    }

    /// リマインダーを追加
    pub fn add_reminder(
        ctx: Context<AddReminder>,
        timestamp: i64,
        note: String,
    ) -> Result<()> {
        let list = &mut ctx.accounts.reminder_list;
        let now  = ctx.accounts.clock.unix_timestamp;

        // 所有者チェック
        if list.owner != ctx.accounts.user.key() {
            return Err(ErrorCode::Unauthorized.into());
        }
        // 過去時刻禁止
        if timestamp <= now {
            return Err(ErrorCode::PastTimestamp.into());
        }
        // メモ長チェック
        if note.len() > 100 {
            return Err(ErrorCode::NoteTooLong.into());
        }
        // 上限チェック
        if list.reminders.len() >= MAX_REMINDERS {
            return Err(ErrorCode::LimitReached.into());
        }
        // 重複チェック
        for entry in list.reminders.iter() {
            if entry.timestamp == timestamp && entry.note == note {
                return Err(ErrorCode::Duplicate.into());
            }
        }
        // 真ブランチで複数処理：追加＋状態更新
        list.reminders.push(ReminderItem { timestamp, note });
        Ok(())
    }

    /// リマインダーを削除
    pub fn remove_reminder(
        ctx: Context<RemoveReminder>,
        timestamp: i64,
        note: String,
    ) -> Result<()> {
        let list = &mut ctx.accounts.reminder_list;

        // 所有者チェック
        if list.owner != ctx.accounts.user.key() {
            return Err(ErrorCode::Unauthorized.into());
        }
        // 探索
        let mut idx: Option<usize> = None;
        for (i, entry) in list.reminders.iter().enumerate() {
            if entry.timestamp == timestamp && entry.note == note {
                idx = Some(i);
                break;
            }
        }
        // 存在チェック・削除
        if let Some(i) = idx {
            list.reminders.remove(i);
            Ok(())
        } else {
            Err(ErrorCode::NotFound.into())
        }
    }
}

#[derive(Accounts)]
pub struct CreateList<'info> {
    /// 二度同じリストを作れない（Reinit Attack 防止）
    #[account(init, payer = user, space = 8 + 32 + 4 + (MAX_REMINDERS * (8 + 4 + 100)))]
    pub reminder_list: Account<'info, ReminderList>,

    /// リスト作成者（署名必須）
    #[account(mut)]
    pub user:          Signer<'info>,

    /// 現在時刻取得用
    pub clock:         Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddReminder<'info> {
    /// 型チェック＆Owner Check
    #[account(mut)]
    pub reminder_list: Account<'info, ReminderList>,

    pub user:          Signer<'info>,
    pub clock:         Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct RemoveReminder<'info> {
    #[account(mut)]
    pub reminder_list: Account<'info, ReminderList>,

    pub user:          Signer<'info>,
}

#[account]
pub struct ReminderList {
    /// このリストを操作できるユーザー
    pub owner:     Pubkey,
    /// リマインダー項目のリスト
    pub reminders: Vec<ReminderItem>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ReminderItem {
    pub timestamp: i64,
    pub note:      String,
}

#[error_code]
pub enum ErrorCode {
    #[msg("権限がありません")]
    Unauthorized,
    #[msg("過去の時刻を指定できません")]
    PastTimestamp,
    #[msg("メモが長すぎます")]
    NoteTooLong,
    #[msg("リマインダーの上限に達しました")]
    LimitReached,
    #[msg("同じリマインダーが既に存在します")]
    Duplicate,
    #[msg("リマインダーが見つかりません")]
    NotFound,
}
