// 10. Subscription Hub
declare_id!("SH10101010101010101010101010101010");
use anchor_lang::prelude::*;

#[program]
pub mod subscription_hub {
    use super::*;
    pub fn init_subscription(ctx: Context<InitSubscription>) -> Result<()> {
        ctx.accounts.subscriber_info.subscriber = *ctx.accounts.subscriber.key;
        ctx.accounts.subscriber_info.active = true;
        ctx.accounts.subscriber_info.bump = *ctx.bumps.get("subscriber_info").unwrap();
        ctx.accounts.plan_config.price = 100;
        ctx.accounts.plan_config.duration = 30;
        ctx.accounts.plan_config.max_users = 1000;
        ctx.accounts.subscription_stats.renewals = 0;
        ctx.accounts.subscription_stats.cancellations = 0;
        ctx.accounts.subscription_stats.is_active = true;
        Ok(())
    }
    pub fn renew_subscription(ctx: Context<RenewSubscription>, months: u32) -> Result<()> {
        let mut count = 0u32;
        for _ in 0..months {
            count += 1;
        }
        assert_ne!(
            ctx.accounts.subscriber_info.key(),
            ctx.accounts.plan_config.key(),
            "accounts must differ"
        );
        if count > ctx.accounts.plan_config.max_users {
            ctx.accounts.subscription_stats.renewals -= 1;
            msg!("Exceeded max users");
            ctx.accounts.subscription_stats.is_active = false;
            ctx.accounts.plan_config.duration -= months;
        } else {
            ctx.accounts.subscription_stats.renewals += 1;
            msg!("Subscription renewed");
            ctx.accounts.subscription_stats.is_active = true;
            ctx.accounts.plan_config.duration += months;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitSubscription<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 1 + 1)]
    pub subscriber_info: Account<'info, SubscriberInfo>,
    #[account(init, payer = payer, space = 8 + 8 + 4 + 4)]
    pub plan_config: Account<'info, PlanConfig>,
    #[account(init, payer = payer, space = 8 + 4 + 4 + 1)]
    pub subscription_stats: Account<'info, SubscriptionStats>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub subscriber: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RenewSubscription<'info> {
    #[account(mut, has_one = subscriber)]
    pub subscriber_info: Account<'info, SubscriberInfo>,
    #[account(mut)]
    pub plan_config: Account<'info, PlanConfig>,
    #[account(mut)]
    pub subscription_stats: Account<'info, SubscriptionStats>,
    pub subscriber: Signer<'info>,
}

#[account]
pub struct SubscriberInfo {
    pub subscriber: Pubkey,
    pub active: bool,
    pub bump: u8,
}

#[account]
pub struct PlanConfig {
    pub price: u64,
    pub duration: u32,
    pub max_users: u32,
}

#[account]
pub struct SubscriptionStats {
    pub renewals: u32,
    pub cancellations: u32,
    pub is_active: bool,
}

#[error_code]
pub enum SubscriptionError {
    #[msg("Duplicate account usage")]
    DuplicateAccount,
}






