use anchor_lang::prelude::*;

declare_id!("TimedRec1313131313131313131313131313131313");

#[program]
pub mod timed_record {
    use super::*;

    /// 初期化：終了時刻を設定
    pub fn init(ctx: Context<InitTime>, end_unix: i64) -> Result<()> {
        let rec = &mut ctx.accounts.rec;
        rec.end_time = end_unix;
        rec.value = 0;
        emit!(TimeInitialized { end_unix });
        Ok(())
    }

    /// 現在時刻が終了前なら更新可能
    pub fn timed_update(ctx: Context<UpdateTime>, val: u64) -> Result<()> {
        let clock = Clock::get()?;
        require!(clock.unix_timestamp < ctx.accounts.rec.end_time, ErrorCode::Expired);
        let rec = &mut ctx.accounts.rec;
        rec.value = val;
        emit!(TimeUpdated { by: ctx.accounts.user.key(), val });
        Ok(())
    }

    /// 終了後のスナップショット取得（読み取りのみ）
    pub fn snapshot(ctx: Context<View>) -> Result<Snapshot> {
        let rec = &ctx.accounts.rec;
        Ok(Snapshot {
            value: rec.value,
            end_time: rec.end_time,
        })
    }
}

#[derive(Accounts)]
pub struct InitTime<'info> {
    #[account(init, payer = payer, space = 8 + 8 + 8)]
    pub rec: Account<'info, TimeRecord>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateTime<'info> {
    #[account(mut)]
    pub rec: Account<'info, TimeRecord>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct View<'info> {
    pub rec: Account<'info, TimeRecord>,
}

#[account]
pub struct TimeRecord {
    pub end_time: i64,
    pub value: u64,
}

#[event]
pub struct TimeInitialized {
    pub end_unix: i64,
}

#[event]
pub struct TimeUpdated {
    pub by: Pubkey,
    pub val: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Snapshot {
    pub value: u64,
    pub end_time: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("許可時間を過ぎています")]
    Expired,
}
