use anchor_lang::prelude::*;

declare_id!("SeasonFlag022222222222222222222222222222222");

#[program]
pub mod season_flags {
    use super::*;

    /// イベント参加フラグを設定
    pub fn mark_participation(ctx: Context<ModifyFlags>, event_id: u8) -> Result<()> {
        let f = &mut ctx.accounts.flags;
        let mask = 1u32.checked_shl(event_id as u32).unwrap_or(0);
        f.participation |= mask;
        f.counts[event_id as usize] = f.counts[event_id as usize].saturating_add(1);
        Ok(())
    }

    /// フラグリセット
    pub fn reset_flags(ctx: Context<ModifyFlags>) -> Result<()> {
        let f = &mut ctx.accounts.flags;
        f.participation = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ModifyFlags<'info> {
    #[account(mut)]
    pub flags: Account<'info, SeasonFlagData>,
}

#[account]
pub struct SeasonFlagData {
    pub participation: u32,    // ビットマスク
    pub counts: [u64; 32],     // 最大32イベント
}
