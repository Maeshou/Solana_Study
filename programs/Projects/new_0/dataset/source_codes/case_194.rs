use anchor_lang::prelude::*;

// ── アカウントデータはファイル冒頭にタプル構造体で定義 ──
#[account]
#[derive(Default)]
pub struct StudySessionManager(pub u8, pub Vec<(u8, u64)>); // (bump, Vec<(subject_id, duration_secs)>)

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzVF");

#[error_code]
pub enum ErrorCode {
    #[msg("Maximum number of sessions reached")]
    MaxSessionsReached,
    #[msg("Session not found")]
    SessionNotFound,
}

#[program]
pub mod study_session_manager {
    use super::*;

    const MAX_SESSIONS: usize = 8;

    /// 初期化：内部 Vec は空、bump のみ設定
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let b = *ctx.bumps.get("manager").unwrap();
        ctx.accounts.manager.0 = b;
        Ok(())
    }

    /// セッション開始：件数制限チェック＋初期 0 で追加
    pub fn start_session(ctx: Context<Modify>, subject_id: u8) -> Result<()> {
        let list = &mut ctx.accounts.manager.1;
        if list.len() >= MAX_SESSIONS {
            return err!(ErrorCode::MaxSessionsReached);
        }
        list.push((subject_id, 0));
        Ok(())
    }

    /// 時間更新：既存セッション検索＋加算
    pub fn add_time(ctx: Context<Modify>, subject_id: u8, secs: u64) -> Result<()> {
        let list = &mut ctx.accounts.manager.1;
        let mut found = false;
        for entry in list.iter_mut() {
            if entry.0 == subject_id {
                entry.1 = entry.1.wrapping_add(secs);
                found = true;
            }
        }
        if found == false {
            return err!(ErrorCode::SessionNotFound);
        }
        Ok(())
    }

    /// セッション削除：該当 subject_id を一括除去
    pub fn end_session(ctx: Context<Modify>, subject_id: u8) -> Result<()> {
        let list = &mut ctx.accounts.manager.1;
        list.retain(|&(sid, _)| {
            if sid == subject_id {
                false
            } else {
                true
            }
        });
        Ok(())
    }

    /// 現在のセッション数をログ出力
    pub fn count_sessions(ctx: Context<Modify>) -> Result<()> {
        let cnt = ctx.accounts.manager.1.len() as u64;
        msg!("Active study sessions: {}", cnt);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_zeroed,
        payer = authority,
        seeds = [b"manager", authority.key().as_ref()],
        bump,
        // discriminator(8) + bump(1) + Vec len(4) + max8*(1+8)
        space = 8 + 1 + 4 + 8 * (1 + 8)
    )]
    pub manager:   Account<'info, StudySessionManager>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Modify<'info> {
    #[account(
        mut,
        seeds = [b"manager", authority.key().as_ref()],
        bump = manager.0,
    )]
    pub manager:   Account<'info, StudySessionManager>,
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
