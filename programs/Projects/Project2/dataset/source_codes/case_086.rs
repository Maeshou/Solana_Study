use anchor_lang::prelude::*;

declare_id!("AccessLog1717171717171717171717171717171717");

#[program]
pub mod access_log {
    use super::*;

    /// ログ記録：呼出者と時刻を履歴に追加
    pub fn record_access(ctx: Context<Record>, info: String) -> Result<()> {
        let rec = &mut ctx.accounts.log;
        let clock = Clock::get()?;
        rec.entries.push(AccessEntry {
            user: ctx.accounts.user.key(),
            timestamp: clock.unix_timestamp,
            note: info.clone(),
        });
        emit!(AccessRecorded {
            user: ctx.accounts.user.key(),
            timestamp: clock.unix_timestamp,
            note: info,
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Record<'info> {
    #[account(mut)]
    pub log: Account<'info, AccessLogData>,
    pub user: Signer<'info>,
}

#[account]
pub struct AccessLogData {
    pub entries: Vec<AccessEntry>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct AccessEntry {
    pub user: Pubkey,
    pub timestamp: i64,
    pub note: String,
}

#[event]
pub struct AccessRecorded {
    pub user: Pubkey,
    pub timestamp: i64,
    pub note: String,
}
