use anchor_lang::prelude::*;

declare_id!("InitAll6666666666666666666666666666666666");

#[program]
pub mod multi_init6 {
    use super::*;

    // サブスクリプション・請求履歴・監査ログを初期化
    pub fn init_subscription(
        ctx: Context<InitSubscription>,
        duration_days: i64,
    ) -> Result<()> {
        let sub = &mut ctx.accounts.subscription;
        let now = Clock::get()?.unix_timestamp;
        sub.user = ctx.accounts.user.key();
        sub.expires = now + duration_days * 86400;
        sub.renewals = 0;

        let billing = &mut ctx.accounts.billing_history;
        billing.entries = Vec::new();
        // 初回請求エントリを追加
        billing.entries.push((now, 1_000)); // 固定料金

        let audit = &mut ctx.accounts.audit_log;
        audit.events = Vec::new();
        audit.events.push(format!("Subscription created at {}", now));
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitSubscription<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8 + 4)]
    pub subscription: Account<'info, SubscriptionData>,
    #[account(init, payer = user, space = 8 + 4 + (12 * 8))]
    pub billing_history: Account<'info, BillingHistoryData>,
    #[account(init, payer = user, space = 8 + 4 + (200 * 2))]
    pub audit_log: Account<'info, AuditLogData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct SubscriptionData {
    pub user: Pubkey,
    pub expires: i64,
    pub renewals: u32,
}

#[account]
pub struct BillingHistoryData {
    pub entries: Vec<(i64, u64)>, // (timestamp, amount)
}

#[account]
pub struct AuditLogData {
    pub events: Vec<String>,
}
