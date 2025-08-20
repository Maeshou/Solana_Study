// =============================================================================
// 17. Subscription Management System
// =============================================================================
#[program]
pub mod secure_subscription {
    use super::*;

    pub fn create_service(ctx: Context<CreateService>, name: String, monthly_price: u64, features: Vec<String>) -> Result<()> {
        let service = &mut ctx.accounts.service;
        service.provider = ctx.accounts.provider.key();
        service.name = name;
        service.monthly_price = monthly_price;
        service.features = features;
        service.is_active = true;
        service.subscriber_count = 0;
        service.bump = *ctx.bumps.get("service").unwrap();
        Ok(())
    }

    pub fn subscribe(ctx: Context<Subscribe>, plan_duration: u8) -> Result<()> {
        let service = &mut ctx.accounts.service;
        let subscription = &mut ctx.accounts.subscription;
        
        require!(service.is_active, SubscriptionError::ServiceNotActive);
        require!(plan_duration > 0 && plan_duration <= 12, SubscriptionError::InvalidDuration);
        
        let current_time = Clock::get()?.unix_timestamp;
        
        subscription.service = service.key();
        subscription.subscriber = ctx.accounts.subscriber.key();
        subscription.start_date = current_time;
        subscription.end_date = current_time + (plan_duration as i64 * 30 * 24 * 60 * 60); // months to seconds
        subscription.monthly_price = service.monthly_price;
        subscription.is_active = true;
        subscription.auto_renew = false;
        subscription.bump = *ctx.bumps.get("subscription").unwrap();
        
        service.subscriber_count += 1;
        
        // Payment for the subscription period
        let total_cost = service.monthly_price * plan_duration as u64;
        **ctx.accounts.subscriber.lamports.borrow_mut() -= total_cost;
        **ctx.accounts.provider.lamports.borrow_mut() += total_cost;
        
        Ok(())
    }

    pub fn renew_subscription(ctx: Context<RenewSubscription>, plan_duration: u8) -> Result<()> {
        let subscription = &mut ctx.accounts.subscription;
        let service = &ctx.accounts.service;
        
        require!(subscription.is_active, SubscriptionError::SubscriptionNotActive);
        require!(plan_duration > 0 && plan_duration <= 12, SubscriptionError::InvalidDuration);
        
        let current_time = Clock::get()?.unix_timestamp;
        let new_end_date = if current_time > subscription.end_date {
            current_time + (plan_duration as i64 * 30 * 24 * 60 * 60)
        } else {
            subscription.end_date + (plan_duration as i64 * 30 * 24 * 60 * 60)
        };
        
        subscription.end_date = new_end_date;
        
        // Payment for renewal
        let total_cost = service.monthly_price * plan_duration as u64;
        **ctx.accounts.subscriber.lamports.borrow_mut() -= total_cost;
        **ctx.accounts.provider.lamports.borrow_mut() += total_cost;
        
        Ok(())
    }

    pub fn cancel_subscription(ctx: Context<CancelSubscription>) -> Result<()> {
        let subscription = &mut ctx.accounts.subscription;
        let service = &mut ctx.accounts.service;
        
        subscription.is_active = false;
        subscription.auto_renew = false;
        service.subscriber_count -= 1;
        
        Ok(())
    }
}

#[account]
pub struct Service {
    pub provider: Pubkey,
    pub name: String,
    pub monthly_price: u64,
    pub features: Vec<String>,
    pub is_active: bool,
    pub subscriber_count: u64,
    pub bump: u8,
}

#[account]
pub struct Subscription {
    pub service: Pubkey,
    pub subscriber: Pubkey,
    pub start_date: i64,
    pub end_date: i64,
    pub monthly_price: u64,
    pub is_active: bool,
    pub auto_renew: bool,
    pub bump: u8,
}

#[derive(Accounts)]
#[instruction(name: String, monthly_price: u64, features: Vec<String>)]
pub struct CreateService<'info> {
    #[account(
        init,
        payer = provider,
        space = 8 + 32 + 4 + name.len() + 8 + 4 + (features.iter().map(|f| 4 + f.len()).sum::<usize>()) + 1 + 8 + 1,
        seeds = [b"service", provider.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub service: Account<'info, Service>,
    
    #[account(mut)]
    pub provider: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Subscribe<'info> {
    #[account(
        mut,
        seeds = [b"service", service.provider.as_ref(), service.name.as_bytes()],
        bump = service.bump
    )]
    pub service: Account<'info, Service>,
    
    #[account(
        init,
        payer = subscriber,
        space = 8 + 32 + 32 + 8 + 8 + 8 + 1 + 1 + 1,
        seeds = [b"subscription", service.key().as_ref(), subscriber.key().as_ref()],
        bump
    )]
    pub subscription: Account<'info, Subscription>,
    
    #[account(mut)]
    pub subscriber: Signer<'info>,
    
    /// CHECK: Verified through service provider field
    #[account(
        mut,
        constraint = provider.key() == service.provider
    )]
    pub provider: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RenewSubscription<'info> {
    #[account(
        seeds = [b"service", service.provider.as_ref(), service.name.as_bytes()],
        bump = service.bump
    )]
    pub service: Account<'info, Service>,
    
    #[account(
        mut,
        seeds = [b"subscription", service.key().as_ref(), subscriber.key().as_ref()],
        bump = subscription.bump,
        constraint = subscription.subscriber == subscriber.key()
    )]
    pub subscription: Account<'info, Subscription>,
    
    #[account(mut)]
    pub subscriber: Signer<'info>,
    
    /// CHECK: Verified through service provider field
    #[account(
        mut,
        constraint = provider.key() == service.provider
    )]
    pub provider: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CancelSubscription<'info> {
    #[account(
        mut,
        seeds = [b"service", service.provider.as_ref(), service.name.as_bytes()],
        bump = service.bump
    )]
    pub service: Account<'info, Service>,
    
    #[account(
        mut,
        seeds = [b"subscription", service.key().as_ref(), subscriber.key().as_ref()],
        bump = subscription.bump,
        constraint = subscription.subscriber == subscriber.key()
    )]
    pub subscription: Account<'info, Subscription>,
    
    pub subscriber: Signer<'info>,
}

#[error_code]
pub enum SubscriptionError {
    #[msg("Service is not active")]
    ServiceNotActive,
    #[msg("Invalid subscription duration")]
    InvalidDuration,
    #[msg("Subscription is not active")]
    SubscriptionNotActive,
}
