// 10) status_initializer: 出品ステータスとスナイプ対策・延長設定
use anchor_lang::prelude::*;

declare_id!("StatInit444444444444444444444444444444");

#[program]
pub mod status_initializer {
    use super::*;

    pub fn init_status(ctx: Context<InitStatus>) -> Result<()> {
        ctx.accounts.state.auction_status = AuctionStatus::Active;
        ctx.accounts.state.auto_extension_enabled = true;
        ctx.accounts.state.extension_time_minutes = 5;
        ctx.accounts.state.snipe_protection_active = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStatus<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + AuctionState::LEN,
        seeds = [b"status", owner.key().as_ref()],
        bump
    )]
    pub state: Account<'info, AuctionState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct AuctionState {
    pub auction_status: AuctionStatus,
    pub auto_extension_enabled: bool,
    pub extension_time_minutes: u32,
    pub snipe_protection_active: bool,
}
impl AuctionState { pub const LEN: usize = 1 + 1 + 4 + 1; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum AuctionStatus { Active, Ended, Cancelled, Suspended }
