use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzV4");

#[account]
#[derive(Default)]
pub struct TempLog {
    pub owner:   Pubkey,             // 所有者
    pub bump:    u8,                 // PDA bump
    pub entries: Vec<(i64, u64)>,    // (timestamp, temp×100) リスト
}

#[error_code]
pub enum ErrorCode {
    #[msg("Maximum number of entries reached")]
    MaxEntriesReached,
    #[msg("Timestamp arithmetic overflow")]
    TimestampOverflow,
}

#[program]
pub mod temp_logger {
    use super::*;

    const MAX_ENTRIES: usize = 10;

    /// アカウント初期化：所有者と bump を設定
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let log = &mut ctx.accounts.log;
        log.owner = ctx.accounts.authority.key();
        log.bump  = *ctx.bumps.get("log").unwrap();
        Ok(())
    }

    /// 温度記録：エントリ数制限＋push
    pub fn record_temperature(
        ctx: Context<ModifyLog>,
        temp_x100: u64,
    ) -> Result<()> {
        let log = &mut ctx.accounts.log;
        let list = &mut log.entries;
        if list.len() >= MAX_ENTRIES {
            return err!(ErrorCode::MaxEntriesReached);
        }
        let now = ctx.accounts.clock.unix_timestamp;
        list.push((now, temp_x100));
        Ok(())
    }

    /// 古いログ削除：checked_sub でオーバーフロー回避
    pub fn purge_old(
        ctx: Context<ModifyLog>,
        age_limit_secs: i64,
    ) -> Result<()> {
        let now = ctx.accounts.clock.unix_timestamp;
        let cutoff = now.checked_sub(age_limit_secs)
            .ok_or(ErrorCode::TimestampOverflow)?;
        ctx.accounts.log.entries.retain(|&(ts, _)| {
            if ts >= cutoff {
                true
            } else {
                false
            }
        });
        Ok(())
    }

    /// 最近の平均温度計算：overflow-safe 演算
    pub fn average_recent(
        ctx: Context<ModifyLog>,
        window_secs: i64,
    ) -> Result<()> {
        let log = &ctx.accounts.log;
        let now = ctx.accounts.clock.unix_timestamp;
        let start = now.checked_sub(window_secs)
            .ok_or(ErrorCode::TimestampOverflow)?;
        let mut sum: u128 = 0;
        let mut cnt: u64 = 0;
        for &(ts, temp) in log.entries.iter() {
            if ts >= start {
                sum = sum.wrapping_add(temp as u128);
                cnt = cnt.wrapping_add(1);
            }
        }
        if cnt > 0 {
            let avg_x100 = (sum / cnt as u128) as u64;
            msg!("Average recent temp: {}.{:02}℃", avg_x100 / 100, avg_x100 % 100);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_zeroed,
        payer = authority,
        seeds = [b"temp_log", authority.key().as_ref()],
        bump,
        space = 8   // discriminator
              + 32  // owner
              + 1   // bump
              + 4   // Vec len prefix
              + 10 * (8 + 8)  // entries max 10*(i64 + u64)
    )]
    pub log:       Account<'info, TempLog>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyLog<'info> {
    #[account(
        mut,
        seeds = [b"temp_log", log.owner.as_ref()],
        bump = log.bump,
        has_one = owner
    )]
    pub log:       Account<'info, TempLog>,
    /// 明示的に所有者の署名をチェック
    #[account(signer)]
    pub owner:     AccountInfo<'info>,
    pub clock:     Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}
