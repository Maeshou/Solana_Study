use anchor_lang::prelude::*;

declare_id!("SafeMulti6666666666666666666666666666666666");

#[program]
pub mod safe_sub {
    use super::*;

    // subscription, billing_history, audit_log をすべて初期化
    pub fn init_subscription(
        ctx: Context<InitSubscription>,
        duration_days: i64,
    ) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let sub = &mut ctx.accounts.subscription;
        sub.user     = ctx.accounts.user.key();
        sub.expires  = now + duration_days * 86400;
        sub.renewals = 0;

        let billing = &mut ctx.accounts.billing_history;
        billing.entries = Vec::new();
        billing.entries.push((now, 1_000));

        let audit = &mut ctx.accounts.audit_log;
        audit.entries = Vec::new();
        audit.entries.push(format!("Subscribed at {}", now));
        Ok(())
    }

    // billing_history と audit_log を mut 更新
    pub fn add_billing(
        ctx: Context<AddBilling>,
        amount: u64,
    ) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let billing = &mut ctx.accounts.billing_history;
        billing.entries.push((now, amount));

        let audit = &mut ctx.accounts.audit_log;
        audit.entries.push(format!("Billed {} at {}", amount, now));
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitSubscription<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8 + 4)]
    pub subscription: Account<'info, SubscriptionData>,
    #[account(init, payer = user, space = 8 + 4 + (12*16))]
    pub billing_history: Account<'info, BillingHistoryData>,
    #[account(init, payer = user, space = 8 + 4 + (200*2))]
    pub audit_log: Account<'info, AuditLogData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddBilling<'info> {
    #[account(mut)] pub billing_history: Account<'info, BillingHistoryData>,
    #[account(mut)] pub audit_log: Account<'info, AuditLogData>,
}

#[account]
pub struct SubscriptionData {
    pub user: Pubkey,
    pub expires: i64,
    pub renewals: u32,
}

#[account]
pub struct BillingHistoryData {
    pub entries: Vec<(i64, u64)>,
}

#[account]
pub struct AuditLogData {
    pub entries: Vec<String>,
}
