
use anchor_lang::prelude::*;

declare_id!("Subscr2222222222222222222222222222222222222");

#[program]
pub mod case2 {
    use super::*;

    pub fn extend_subscription(ctx: Context<ExtendSubscription>, days: u64) -> Result<()> {
        let sub = &mut ctx.accounts.subscription;
        let before = sub.expires;
        sub.expires = sub.expires.saturating_add(days);
        msg!("Subscription extended from {} to {}", before, sub.expires);
        sub.action_log.push(format!("Extended by {} days", days));
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExtendSubscription<'info> {
    #[account(mut)]
    pub subscription: Account<'info, Subscription>,
    /// CHECK: user is not validated and not required to sign
    pub user: UncheckedAccount<'info>,
}

#[account]
pub struct Subscription {
    pub expires: u64,
    pub user: Pubkey,
    pub action_log: Vec<String>,
}
