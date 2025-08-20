// 3. セッション管理＋タイムスタンプログ
use anchor_lang::prelude::*;
declare_id!("SESS111122223333444455556666777788");

#[program]
pub mod misinit_session_v4 {
    use super::*;

    pub fn init_session(
        ctx: Context<InitSession>,
        token: String,
        ttl: i64,
    ) -> Result<()> {
        let ss = &mut ctx.accounts.session;
        ss.token = token;
        ss.active = true;
        ss.expires_at = Clock::get()?.unix_timestamp + ttl;
        Ok(())
    }

    pub fn refresh_session(
        ctx: Context<InitSession>,
    ) -> Result<()> {
        let ss = &mut ctx.accounts.session;
        require!(ss.active, ErrorCode::SessionInactive);
        ss.expires_at = Clock::get()?.unix_timestamp + (ss.expires_at - ss.created_at);
        Ok(())
    }

    pub fn log_timestamp(
        ctx: Context<InitSession>,
        ts: i64,
    ) -> Result<()> {
        let tl = &mut ctx.accounts.time_log;
        tl.timestamps.push(ts);
        // 100件を超えたら古いものを削除
        if tl.timestamps.len() > 100 { tl.timestamps.remove(0); }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitSession<'info> {
    #[account(init, payer = signer, space = 8 + (4 + 64) + 1 + 8 + 8)]
    pub session: Account<'info, SessionData>,

    #[account(mut)]
    pub time_log: Account<'info, TimeLog>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct SessionData {
    pub token: String,
    pub active: bool,
    pub created_at: i64,
    pub expires_at: i64,
}

#[account]
pub struct TimeLog {
    pub timestamps: Vec<i64>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("セッションが無効です。")]
    SessionInactive,
}
