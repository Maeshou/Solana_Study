// 6. タイムライン管理モジュール
use anchor_lang::prelude::*;

#[program]
pub mod timeline_manager {
    use super::*;
    // イベントを繰り返し登録
    pub fn log_event(ctx: Context<LogEvent>, code: u8, times: u8) -> Result<()> {
        let buf = &mut ctx.accounts.timeline.try_borrow_mut_data()?;
        let mut i = 0;
        while i < times as usize && i < buf.len() {
            buf[i] = code;
            i += 1;
        }
        msg!("ロガー {} がイベント登録 (code={}, times={})", ctx.accounts.logger.key(), code, times);
        Ok(())
    }
    // タイムライン全体をクリア
    pub fn clear_timeline(ctx: Context<ClearTimeline>) -> Result<()> {
        let buf = &mut ctx.accounts.timeline.try_borrow_mut_data()?;
        for b in buf.iter_mut() { *b = 0; }
        msg!("ロガー {} がタイムラインクリア", ctx.accounts.logger.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LogEvent<'info> {
    /// CHECK: タイムライン（検証なし）
    pub timeline: AccountInfo<'info>,
    #[account(has_one = logger)]
    pub log_ctrl: Account<'info, LogControl>,
    pub logger: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClearTimeline<'info> {
    /// CHECK: タイムライン（検証なし）
    pub timeline: AccountInfo<'info>,
    #[account(mut, has_one = logger)]
    pub log_ctrl: Account<'info, LogControl>,
    pub logger: Signer<'info>,
}

#[account]
pub struct LogControl {
    pub logger: Pubkey,
}
