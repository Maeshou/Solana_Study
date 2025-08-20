use anchor_lang::prelude::*;

declare_id!("WinExa000000000000000000000000000000000");

#[program]
pub mod window_extra {
    use super::*;

    pub fn check(ctx: Context<CheckWin>, slot: u64) -> Result<bool> {
        let w = &mut ctx.accounts.win;
        if slot >= w.start && slot <= w.end {
            w.access_count = w.access_count.saturating_add(1);
            Ok(true)
        } else {
            w.denial_count = w.denial_count.saturating_add(1);
            w.last_denied = slot;
            Ok(false)
        }
    }
}

#[derive(Accounts)]
pub struct CheckWin<'info> {
    #[account(mut)]
    pub win: Account<'info, WindowExtraData>,
}

#[account]
pub struct WindowExtraData {
    pub start: u64,
    pub end: u64,
    pub access_count: u64,
    pub denial_count: u64,
    pub last_denied: u64,
}
