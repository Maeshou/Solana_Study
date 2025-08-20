use anchor_lang::prelude::*;

declare_id!("Work111111111111111111111111111111111111");

#[program]
pub mod workout_logger {
    /// 新規セッション記録
    pub fn create_session(
        ctx: Context<CreateSession>,
        timestamp: i64,
        duration_minutes: u16,
        note: String,
    ) -> Result<()> {
        // バリデーション
        if duration_minutes == 0 {
            return Err(ErrorCode::InvalidDuration.into());
        }
        if note.len() > 128 {
            return Err(ErrorCode::NoteTooLong.into());
        }

        let sess = &mut ctx.accounts.session;
        sess.owner            = ctx.accounts.user.key();
        sess.timestamp        = timestamp;
        sess.duration_minutes = duration_minutes;
        sess.note             = note;
        Ok(())
    }

    /// 既存セッションを更新
    pub fn update_session(
        ctx: Context<UpdateSession>,
        new_timestamp: i64,
        new_duration_minutes: u16,
        new_note: String,
    ) -> Result<()> {
        // バリデーション
        if new_duration_minutes == 0 {
            return Err(ErrorCode::InvalidDuration.into());
        }
        if new_note.len() > 128 {
            return Err(ErrorCode::NoteTooLong.into());
        }

        let sess = &mut ctx.accounts.session;
        // 所有者チェック
        if sess.owner != ctx.accounts.user.key() {
            return Err(ErrorCode::Unauthorized.into());
        }

        sess.timestamp        = new_timestamp;
        sess.duration_minutes = new_duration_minutes;
        sess.note             = new_note;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateSession<'info> {
    /// init 制約で同一アカウント再初期化防止（Reinit Attack）
    #[account(init, payer = user, space = 8 + 32 + 8 + 2 + 4 + 128)]
    pub session: Account<'info, Session>,

    /// このトランザクションを署名したユーザー（Signer Authorization）
    #[account(mut)]
    pub user:    Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateSession<'info> {
    /// Account<> による Owner Check & Type Cosplay
    #[account(mut)]
    pub session: Account<'info, Session>,

    /// 実際に署名したユーザー
    pub user:    Signer<'info>,
}

#[account]
pub struct Session {
    /// この記録を操作できるユーザー
    pub owner:            Pubkey,
    /// 記録タイムスタンプ (UNIX)
    pub timestamp:        i64,
    /// セッション時間 (分)
    pub duration_minutes: u16,
    /// メモ (最大128文字)
    pub note:             String,
}

#[error_code]
pub enum ErrorCode {
    #[msg("不正な duration です")]
    InvalidDuration,
    #[msg("メモが長すぎます")]
    NoteTooLong,
    #[msg("権限がありません")]
    Unauthorized,
}
