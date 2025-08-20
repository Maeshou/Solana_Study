use anchor_lang::prelude::*;

declare_id!("VulnEx39000000000000000000000000000000000039");

#[program]
pub mod example39 {
    pub fn pause_subscription(ctx: Context<Ctx39>) -> Result<()> {
        // pause_log は所有者検証なし
        ctx.accounts.pause_log.data.borrow_mut().extend_from_slice(b"paused");
        // subscription_state は has_one で subscriber 検証済み
        let sub = &mut ctx.accounts.subscription_state;
        sub.active = false;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx39<'info> {
    pub clock: Sysvar<'info, Clock>,
    #[account(mut)]
    pub pause_log: AccountInfo<'info>,
    #[account(mut, has_one = subscriber)]
    pub subscription_state: Account<'info, SubscriptionState>,
    pub subscriber: Signer<'info>,
}

#[account]
pub struct SubscriptionState {
    pub subscriber: Pubkey,
    pub active: bool,
}
