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

        // 権限・入力チェックをまとめて require! で
        require!(list.owner == ctx.accounts.user.key(), ErrorCode::Unauthorized);
        require!(timestamp > now, ErrorCode::PastTimestamp);
        require!(note.len() <= 100, ErrorCode::NoteTooLong);
        require!(list.reminders.len() < MAX_REMINDERS, ErrorCode::LimitReached);

        // 重複チェック
        let mut duplicate = false;
        for entry in list.reminders.iter() {
            if entry.timestamp == timestamp && entry.note == note {
                duplicate = true;
                break;
            }
        }
        require!(!duplicate, ErrorCode::Duplicate);

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
        require!(list.owner == ctx.accounts.user.key(), ErrorCode::Unauthorized);

        // インデックス探索
        let mut idx: Option<usize> = None;
        for (i, entry) in list.reminders.iter().enumerate() {
            if entry.timestamp == timestamp && entry.note == note {
                idx = Some(i);
                break;
            }
        }
        let i = idx.ok_or(ErrorCode::NotFound)?;
        list.reminders.remove(i);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateList<'info> {
    #[account(init, payer = user, space = 8 + 32 + 4 + (MAX_REMINDERS * (8 + 4 + 100)))]
    pub reminder_list: Account<'info, ReminderList>,
    #[account(mut)] pub user: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddReminder<'info> {
    #[account(mut)] pub reminder_list: Account<'info, ReminderList>,
    pub user: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct RemoveReminder<'info> {
    #[account(mut)] pub reminder_list: Account<'info, ReminderList>,
    pub user: Signer<'info>,
}

#[account]
pub struct ReminderList {
    pub owner:     Pubkey,
    pub reminders: Vec<ReminderItem>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ReminderItem {
    pub timestamp: i64,
    pub note:      String,
}

#[error_code]
pub enum ErrorCode {
    #[msg("権限がありません")] Unauthorized,
    #[msg("過去の時刻を指定できません")] PastTimestamp,
    #[msg("メモが長すぎます")] NoteTooLong,
    #[msg("リマインダーの上限に達しました")] LimitReached,
    #[msg("同じリマインダーが既に存在します")] Duplicate,
    #[msg("リマインダーが見つかりません")] NotFound,
}
