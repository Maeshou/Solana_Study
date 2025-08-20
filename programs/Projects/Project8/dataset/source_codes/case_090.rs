// 8) constraints_guard: 価格・期間バリデーション＋時間の確定（ネストifで && 回避）
use anchor_lang::prelude::*;

declare_id!("CnstrGd444444444444444444444444444444");

#[program]
pub mod constraints_guard {
    use super::*;

    pub fn validate_and_fix_times(
        ctx: Context<Validate>,
        starting_price: u64,
        duration_secs: u64,
    ) -> Result<()> {
        // 価格レンジ
        require!(starting_price >= 1_000, GuardError::StartingPriceTooLow);

        // 期間レンジ（1時間～7日）
        require!(duration_secs >= 3_600, GuardError::AuctionTooShort);
        let mut valid_duration = duration_secs;
        if duration_secs > 604_800 {
            valid_duration = 604_800;
        }

        let now = Clock::get()?.unix_timestamp;
        ctx.accounts.window.auction_start_time = now;
        ctx.accounts.window.auction_end_time = now + valid_duration as i64;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Validate<'info> {
    #[account(
        init,
        payer = lister,
        space = 8 + TimeWindow::LEN,
        seeds = [b"window", lister.key().as_ref()],
        bump
    )]
    pub window: Account<'info, TimeWindow>,
    #[account(mut)]
    pub lister: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct TimeWindow {
    pub auction_start_time: i64,
    pub auction_end_time: i64,
}
impl TimeWindow { pub const LEN: usize = 8 + 8; }

#[error_code]
pub enum GuardError {
    #[msg("Starting price is too low")] StartingPriceTooLow,
    #[msg("Auction duration is too short")] AuctionTooShort,
}
