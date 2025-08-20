
// 9. Subscription Service with Recurring Payments
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};

declare_id!("SubscriptionSvc1111111111111111111111111111111111");

#[program]
pub mod subscription_service {
    use super::*;
    
    pub fn create_subscription_plan(ctx: Context<CreateSubscriptionPlan>, price: u64, interval: i64) -> Result<()> {
        let plan = &mut ctx.accounts.plan;
        plan.provider = ctx.accounts.provider.key();
        plan.price = price;
        plan.interval = interval;
        plan.active_subscriptions = 0;
        Ok(())
    }
    
    pub fn subscribe(ctx: Context<Subscribe>) -> Result<()> {
        let plan = &ctx.accounts.plan;
        let subscription = &mut ctx.accounts.subscription;
        
        subscription.subscriber = ctx.accounts.subscriber.key();
        subscription.plan = plan.key();
        subscription.start_time = Clock::get()?.unix_timestamp;
        subscription.next_payment = Clock::get()?.unix_timestamp + plan.interval;
        subscription.is_active = true;
        
        // Make initial payment
        let cpi_accounts = anchor_spl::token::Transfer {
            from: ctx.accounts.subscriber_token_account.to_account_info(),
            to: ctx.accounts.provider_token_account.to_account_info(),
            authority: ctx.accounts.subscriber.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        anchor_spl::token::transfer(cpi_ctx, plan.price)?;
        
        Ok(())
    }
    
    pub fn process_payment(ctx: Context<ProcessPayment>) -> Result<()> {
        let plan = &ctx.accounts.plan;
        let subscription = &mut ctx.accounts.subscription;
        
        require!(subscription.is_active, SubscriptionError::InactiveSubscription);
        require!(Clock::get()?.unix_timestamp >= subscription.next_payment, SubscriptionError::PaymentNotDue);
        
        // Process payment
        let cpi_accounts = anchor_spl::token::Transfer {
            from: ctx.accounts.subscriber_token_account.to_account_info(),
            to: ctx.accounts.provider_token_account.to_account_info(),
            authority: ctx.accounts.subscriber.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        anchor_spl::token::transfer(cpi_ctx, plan.price)?;
        
        subscription.next_payment = Clock::get()?.unix_timestamp + plan.interval;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateSubscriptionPlan<'info> {
    #[account(init, payer = provider, space = 8 + 100)]
    pub plan: Account<'info, SubscriptionPlan>,
    #[account(mut)]
    pub provider: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Subscribe<'info> {
    pub plan: Account<'info, SubscriptionPlan>,
    #[account(init, payer = subscriber, space = 8 + 150, seeds = [b"subscription", subscriber.key().as_ref(), plan.key().as_ref()], bump)]
    pub subscription: Account<'info, Subscription>,
    #[account(mut)]
    pub subscriber_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub provider_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub subscriber: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessPayment<'info> {
    pub plan: Account<'info, SubscriptionPlan>,
    #[account(mut, seeds = [b"subscription", subscription.subscriber.as_ref(), plan.key().as_ref()], bump)]
    pub subscription: Account<'info, Subscription>,
    #[account(mut)]
    pub subscriber_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub provider_token_account: Account<'info, TokenAccount>,
    pub subscriber: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct SubscriptionPlan {
    pub provider: Pubkey,
    pub price: u64,
    pub interval: i64,
    pub active_subscriptions: u64,
}

#[account]
pub struct Subscription {
    pub subscriber: Pubkey,
    pub plan: Pubkey,
    pub start_time: i64,
    pub next_payment: i64,
    pub is_active: bool,
}

#[error_code]
pub enum SubscriptionError {
    #[msg("Subscription is not active")]
    InactiveSubscription,
    #[msg("Payment is not due yet")]
    PaymentNotDue,
}
