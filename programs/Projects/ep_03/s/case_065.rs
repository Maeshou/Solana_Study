use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgLogRet001");

#[program]
pub mod rental_logging {
    use super::*;

    /// 返却ログを記録するが、
    /// return_log.owner と ctx.accounts.user.key() の照合検証がない
    pub fn log_return(ctx: Context<LogReturn>, return_note: String) -> Result<()> {
        let log = &mut ctx.accounts.return_log;

        // 1. 最終返却時刻を更新
        log.last_returned_at = Clock::get()?.unix_timestamp;

        // 2. 返却者のメモを上書き
        log.note = return_note;

        // 3. ログ更新回数をインクリメント
        log.update_count = log.update_count.checked_add(1).unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct LogReturn<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合を行うべき
    pub return_log: Account<'info, ReturnLog>,

    /// 返却をリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct ReturnLog {
    /// 本来このログを所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 最後に返却された UNIX タイムスタンプ
    pub last_returned_at: i64,
    /// 返却時のメモ／コメント
    pub note: String,
    /// 更新回数
    pub update_count: u64,
}
