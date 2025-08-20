// 4) watchers_initializer: ウォッチャー20枠の事前割当（分岐なし、ループは長め）
use anchor_lang::prelude::*;

declare_id!("WatchInit444444444444444444444444444444");

#[program]
pub mod watchers_initializer {
    use super::*;

    pub fn init_watchers(ctx: Context<InitWatchers>, slots: u32) -> Result<()> {
        let mut list: Vec<WatcherInfo> = Vec::new();
        let mut i = 0u32;
        let count = slots.max(20);
        while i < count {
            list.push(WatcherInfo {
                watcher_pubkey: None,
                watch_start_time: 0,
                notification_preferences: NotificationPreferences {
                    bid_outbid_alert: false,
                    auction_ending_alert: false,
                    price_threshold_alert: false,
                    threshold_amount: 0,
                },
            });
            i = i.saturating_add(1);
        }
        ctx.accounts.watchbook.watchers = list;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitWatchers<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + WatchBook::LEN,
        seeds = [b"watch", creator.key().as_ref()],
        bump
    )]
    pub watchbook: Account<'info, WatchBook>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct WatchBook { pub watchers: Vec<WatcherInfo> }
impl WatchBook { pub const LEN: usize = 4 + 32 * WatcherInfo::LEN; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct WatcherInfo {
    pub watcher_pubkey: Option<Pubkey>,
    pub watch_start_time: i64,
    pub notification_preferences: NotificationPreferences,
}
impl WatcherInfo { pub const LEN: usize = 33 + 8 + NotificationPreferences::LEN; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct NotificationPreferences {
    pub bid_outbid_alert: bool,
    pub auction_ending_alert: bool,
    pub price_threshold_alert: bool,
    pub threshold_amount: u64,
}
impl NotificationPreferences { pub const LEN: usize = 1 + 1 + 1 + 8; }
