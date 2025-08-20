use anchor_lang::prelude::*;

// ── アカウントデータはファイル冒頭にタプル構造体で定義 ──
#[account]
#[derive(Default)]
pub struct TempLog(pub u8, pub Vec<(i64, u64)>); // (bump, Vec<(timestamp, temp×100)>)

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzV4");

#[error_code]
pub enum ErrorCode {
    #[msg("Maximum number of entries reached")]
    MaxEntriesReached,
}

#[program]
pub mod temp_logger {
    use super::*;

    const MAX_ENTRIES: usize = 10;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // PDA を authority キーと結びつけることで、
        // あとは seeds チェックだけでオーナー認証を担保
        Ok(())
    }

    pub fn record_temperature(
        ctx: Context<ModifyLog>,
        temp_x100: u64,
    ) -> Result<()> {
        let list = &mut ctx.accounts.log.1;
        if list.len() >= MAX_ENTRIES {
            return err!(ErrorCode::MaxEntriesReached);
        }
        let now = ctx.accounts.clock.unix_timestamp;
        list.push((now, temp_x100));
        Ok(())
    }

    pub fn purge_old(
        ctx: Context<ModifyLog>,
        age_limit_secs: i64,
    ) -> Result<()> {
        let now = ctx.accounts.clock.unix_timestamp;
        ctx.accounts
            .log
            .1
            .retain(|&(ts, _)| now - ts <= age_limit_secs);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        seeds = [b"temp_log", authority.key().as_ref()],
        bump,
        space = 8 + 1 + 4 + 10*(8+8)
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
        seeds = [b"temp_log", authority.key().as_ref()],
        bump = log.0,
    )]
    pub log:       Account<'info, TempLog>,

    /// PDA の seeds に含めた authority キーでオーナー認証完結
    #[account(signer)]
    pub authority: AccountInfo<'info>,

    pub clock:     Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}
