use anchor_lang::prelude::*;

declare_id!("Sched11111111111111111111111111111111111");

#[program]
pub mod event_scheduler {
    /// 新しいイベントを作成
    pub fn create_event(
        ctx: Context<CreateEvent>,
        timestamp: i64,
        location: String,
        details: String,
    ) -> Result<()> {
        // バリデーション
        if location.len() > 64 {
            return Err(ErrorCode::LocationTooLong.into());
        }
        if details.len() > 256 {
            return Err(ErrorCode::DetailsTooLong.into());
        }

        let ev = &mut ctx.accounts.event;
        ev.owner     = ctx.accounts.user.key();  // Signer Authorization
        ev.timestamp = timestamp;
        ev.location  = location;
        ev.details   = details;
        Ok(())
    }

    /// 既存イベントを更新
    pub fn update_event(
        ctx: Context<UpdateEvent>,
        new_timestamp: i64,
        new_location: String,
        new_details: String,
    ) -> Result<()> {
        // バリデーション
        if new_location.len() > 64 {
            return Err(ErrorCode::LocationTooLong.into());
        }
        if new_details.len() > 256 {
            return Err(ErrorCode::DetailsTooLong.into());
        }

        let ev = &mut ctx.accounts.event;
        // 所有者チェック
        if ev.owner != ctx.accounts.user.key() {
            return Err(ErrorCode::Unauthorized.into());
        }

        ev.timestamp = new_timestamp;
        ev.location  = new_location;
        ev.details   = new_details;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateEvent<'info> {
    /// 同一アカウントを二度初期化できない（Reinit Attack 防止）
    #[account(init, payer = user, space = 8 + 32 + 8 + 4 + 64 + 4 + 256)]
    pub event:           Account<'info, EventAccount>,

    /// 操作するユーザー（Signer Authorization）
    #[account(mut)]
    pub user:            Signer<'info>,

    pub system_program:  Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateEvent<'info> {
    /// Anchor の Account<> による Owner Check & Type Cosplay
    #[account(mut)]
    pub event:           Account<'info, EventAccount>,

    /// 実際に署名したユーザー
    pub user:            Signer<'info>,
}

#[account]
pub struct EventAccount {
    /// このイベントを操作できるユーザー
    pub owner:     Pubkey,
    /// イベントの日時 (UNIX timestamp)
    pub timestamp: i64,
    /// 開催場所（最大64文字）
    pub location:  String,
    /// 詳細説明（最大256文字）
    pub details:   String,
}

#[error_code]
pub enum ErrorCode {
    #[msg("権限がありません")]
    Unauthorized,
    #[msg("場所の文字数が長すぎます")]
    LocationTooLong,
    #[msg("詳細の文字数が長すぎます")]
    DetailsTooLong,
}
