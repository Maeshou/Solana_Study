use anchor_lang::prelude::*;

declare_id!("OpLog47474747474747474747474747474747");

#[program]
pub mod op_log47 {
    use super::*;

    /// ログに文字列を追加
    pub fn log_entry(ctx: Context<LogOp>, entry: String) -> Result<()> {
        let ld = &mut ctx.accounts.log;
        ld.entries.push(entry);
        Ok(())
    }

    /// ログを全消去
    pub fn clear_log(ctx: Context<LogOp>) -> Result<()> {
        let ld = &mut ctx.accounts.log;
        ld.entries.clear();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LogOp<'info> {
    #[account(mut)]
    pub log: Account<'info, LogData>,
}

#[account]
pub struct LogData {
    pub entries: Vec<String>,
}
