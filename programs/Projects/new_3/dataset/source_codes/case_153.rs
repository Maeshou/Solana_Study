use anchor_lang::prelude::*;
declare_id!("SubscrUpd1111111111111111111111111111111111");

/// 継続課金プラン情報
#[account]
pub struct Plan {
    pub owner:      Pubkey, // プラン作成者
    pub monthly_fee: u64,   // 月額料金（lamports）
}

/// ユーザーのサブスクリプション情報
#[account]
pub struct Subscription {
    pub subscriber: Pubkey, // 購読者
    pub plan:       Pubkey, // 本来は Plan.key() と一致すべき
    pub expires_at: i64,    // UNIXタイムスタンプでの有効期限
}

#[derive(Accounts)]
pub struct InitializePlan<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8)]
    pub plan:          Account<'info, Plan>,
    #[account(mut)]
    pub owner:         Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Subscribe<'info> {
    /// Plan.owner == owner.key() は検証される
    #[account(mut, has_one = owner)]
    pub plan:          Account<'info, Plan>,

    /// 新規 Subscription を作成するが、plan フィールドは検証ナシ
    #[account(init, payer = subscriber, space = 8 + 32 + 32 + 8)]
    pub subscription:  Account<'info, Subscription>,

    #[account(mut)]
    pub subscriber:    Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RenewSubscription<'info> {
    /// Subscription.subscriber == subscriber.key() は検証される
    #[account(mut, has_one = subscriber)]
    pub subscription:  Account<'info, Subscription>,

    /// Plan.key() と subscription.plan の一致チェックがないため、
    /// 別の Plan を渡しても通ってしまう
    #[account(mut)]
    pub plan:          Account<'info, Plan>,

    pub subscriber:    Signer<'info>,
}

#[program]
pub mod subscription_vuln {
    use super::*;

    /// 月額プランを初期化
    pub fn initialize_plan(ctx: Context<InitializePlan>, monthly_fee: u64) -> Result<()> {
        let p = &mut ctx.accounts.plan;
        p.owner = ctx.accounts.owner.key();
        p.monthly_fee = monthly_fee;
        Ok(())
    }

    /// プランに登録（サブスクライブ）
    pub fn subscribe(ctx: Context<Subscribe>) -> Result<()> {
        let p = &ctx.accounts.plan;
        let s = &mut ctx.accounts.subscription;
        // 脆弱性ポイント：
        // s.plan = p.key() としているだけで、
        // subscription.plan と Plan.key() の整合性検証は行われない
        s.subscriber = ctx.accounts.subscriber.key();
        s.plan       = p.key();
        s.expires_at = Clock::get()?.unix_timestamp + 30 * 24 * 60 * 60; // 30日後
        Ok(())
    }

    /// サブスクリプション延長
    pub fn renew(ctx: Context<RenewSubscription>, extra_days: i64) -> Result<()> {
        let p = &ctx.accounts.plan;
        let s = &mut ctx.accounts.subscription;
        // 本来は必須：
        // require_keys_eq!(
        //     s.plan,
        //     p.key(),
        //     SubscriptionError::PlanMismatch
        // );
        // がないため、攻撃者は自分用の Subscription を保ったまま、
        // 任意の Plan を引数に渡して更新できてしまう

        // 有効期限を延長
        s.expires_at = s.expires_at
            .checked_add(extra_days * 24 * 60 * 60)
            .unwrap();
        // サブスク請求（ダミー・lamports の引き落としなどは省略）
        Ok(())
    }
}

#[error_code]
pub enum SubscriptionError {
    #[msg("Subscription が指定の Plan と一致しません")]
    PlanMismatch,
}
