use anchor_lang::prelude::*;
declare_id!("SubscrSafe1111111111111111111111111111111");

/// プラン情報
#[account]
pub struct Plan {
    pub provider:    Pubkey,  // プラン提供者
    pub name:        String,  // プラン名
    pub interval:    u64,     // 課金間隔（日数）
}

/// サブスクリプション情報
#[account]
pub struct Subscription {
    pub subscriber:  Pubkey,  // 購読者
    pub plan:        Pubkey,  // Plan.key()
    pub expires_at:  i64,     // 有効期限（UNIXタイム）
}

#[derive(Accounts)]
pub struct CreatePlan<'info> {
    #[account(init, payer = provider, space = 8 + 32 + 4 + 64 + 8)]
    pub plan:        Account<'info, Plan>,
    #[account(mut)]
    pub provider:    Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Subscribe<'info> {
    /// Plan.provider == provider.key() を検証
    #[account(mut, has_one = provider)]
    pub plan:        Account<'info, Plan>,

    /// Subscription.plan == plan.key()、Subscription.subscriber == subscriber.key() を検証
    #[account(
        init,
        payer = subscriber,
        space = 8 + 32 + 32 + 8,
        has_one = plan,
        has_one = subscriber
    )]
    pub subscription: Account<'info, Subscription>,

    #[account(mut)]
    pub provider:    Signer<'info>,
    #[account(mut)]
    pub subscriber:  Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RenewSubscription<'info> {
    /// Subscription.subscriber == subscriber.key()、Subscription.plan == plan.key() を検証
    #[account(mut, has_one = subscriber, has_one = plan)]
    pub subscription: Account<'info, Subscription>,

    /// Plan.account を指定（キー一致は上の has_one で担保）
    #[account(mut)]
    pub plan:        Account<'info, Plan>,

    #[account(mut)]
    pub subscriber:  Signer<'info>,
}

#[program]
pub mod subscription_safe {
    use super::*;

    /// プランを作成
    pub fn create_plan(
        ctx: Context<CreatePlan>,
        name: String,
        interval_days: u64
    ) -> Result<()> {
        let p = &mut ctx.accounts.plan;
        p.provider = ctx.accounts.provider.key();
        p.name     = name;
        p.interval = interval_days;
        Ok(())
    }

    /// サブスク登録
    pub fn subscribe(
        ctx: Context<Subscribe>,
        duration_seconds: i64
    ) -> Result<()> {
        let plan = &ctx.accounts.plan;
        let sub  = &mut ctx.accounts.subscription;

        // 明示的にフィールドをセット
        sub.subscriber = ctx.accounts.subscriber.key();
        sub.plan       = plan.key();
        sub.expires_at = Clock::get()?.unix_timestamp + duration_seconds;

        // 二重チェック（optional）
        require_keys_eq!(sub.plan, plan.key(), SubError::PlanMismatch);
        require_keys_eq!(sub.subscriber, ctx.accounts.subscriber.key(), SubError::SubscriberMismatch);

        Ok(())
    }

    /// サブスク更新（有効期限延長）
    pub fn renew(
        ctx: Context<RenewSubscription>
    ) -> Result<()> {
        let plan = &ctx.accounts.plan;
        let sub  = &mut ctx.accounts.subscription;

        // 二重チェック
        require_keys_eq!(sub.plan, plan.key(), SubError::PlanMismatch);
        require_keys_eq!(sub.subscriber, ctx.accounts.subscriber.key(), SubError::SubscriberMismatch);

        // interval_days を秒に換算して有効期限を延長
        let extension = (plan.interval as i64) * 86_400;
        sub.expires_at = sub.expires_at.saturating_add(extension);
        Ok(())
    }
}

#[error_code]
pub enum SubError {
    #[msg("Subscription.plan が Plan と一致しません")]
    PlanMismatch,
    #[msg("Subscription.subscriber が Subscriber と一致しません")]
    SubscriberMismatch,
}
