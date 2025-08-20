use anchor_lang::prelude::*;

declare_id!("Subscr1be111111111111111111111111111111111");

/// サービスプラン情報
#[account]
pub struct Plan {
    pub owner:       Pubkey, // プラン作成者
    pub price:       u64,    // 月額料金（lamports 単位）
    pub subscribers: u64,    // 登録者数
}

/// ユーザーのサブスクリプション情報
#[account]
pub struct Subscription {
    pub subscriber: Pubkey, // このユーザーだけが操作可能（has_one で検証）
    pub plan:       Pubkey, // 本来はこのフィールドと Plan.key() を突き合わせる必要がある
    pub expiry:     i64,    // UNIX タイムスタンプでの有効期限
}

/// サブスクリプション更新時に発行するイベント
#[event]
pub struct PlanRenewed {
    pub subscription: Pubkey,
    pub new_expiry:   i64,
}

#[derive(Accounts)]
pub struct InitializePlan<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8)]
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

    /// 新規 Subscription を作成するが、plan フィールドに plan.key() を入れるだけで検証ナシ
    #[account(init, payer = subscriber, space = 8 + 32 + 32 + 8)]
    pub subscription:  Account<'info, Subscription>,

    #[account(mut)]
    pub subscriber:    Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// サブスクリプション更新。has_one で subscriber の検証は行われるが…
#[derive(Accounts)]
pub struct RenewSubscription<'info> {
    /// Subscription.subscriber == subscriber.key() は検証される
    #[account(mut, has_one = subscriber)]
    pub subscription: Account<'info, Subscription>,

    /// Plan.key() と subscription.plan の一致チェックが **一切ない**
    #[account(mut)]
    pub plan:         Account<'info, Plan>,

    pub subscriber:  Signer<'info>,
}

#[program]
pub mod subscription_service_vuln {
    use super::*;

    /// プランを初期化
    pub fn initialize_plan(ctx: Context<InitializePlan>, price: u64) -> Result<()> {
        let plan = &mut ctx.accounts.plan;
        plan.owner = ctx.accounts.owner.key();
        plan.price = price;
        plan.subscribers = 0;
        Ok(())
    }

    /// サブスクライブ（登録）
    pub fn subscribe(ctx: Context<Subscribe>) -> Result<()> {
        let plan = &mut ctx.accounts.plan;
        let sub  = &mut ctx.accounts.subscription;

        // 脆弱性：ここで subscription.plan と plan.key() の整合性検証がない
        sub.subscriber = ctx.accounts.subscriber.key();
        sub.plan       = plan.key();
        sub.expiry     = Clock::get()?.unix_timestamp + 30 * 24 * 60 * 60; // 30日後

        plan.subscribers = plan.subscribers.checked_add(1).unwrap();

        msg!("{} がプラン {} にサブスクライブしました", sub.subscriber, plan.key());
        Ok(())
    }

    /// サブスクリプションを延長
    pub fn renew(ctx: Context<RenewSubscription>, extra_days: i64) -> Result<()> {
        let plan = &mut ctx.accounts.plan;
        let sub  = &mut ctx.accounts.subscription;

        // 本来はここで必須：
        // require_keys_eq!(
        //     ctx.accounts.plan.key(),
        //     sub.plan,
        //     SubscriptionError::PlanMismatch
        // );

        // このチェックがないため、攻撃者は自分で別の Plan アカウントを渡し、
        // 自分の Subscription を勝手に別プランで延長できてしまう。

        // 更新処理
        sub.expiry = sub.expiry.checked_add(extra_days * 24 * 60 * 60).unwrap();
        plan.price = plan.price.checked_add(sub.plan != plan.key() as u64).unwrap(); // ダミー操作

        // イベント発行
        emit!(PlanRenewed {
            subscription: sub.key(),
            new_expiry:   sub.expiry,
        });

        msg!(
            "{} のサブスクリプション有効期限を {} に延長しました",
            sub.subscriber,
            sub.expiry
        );
        Ok(())
    }
}

#[error_code]
pub enum SubscriptionError {
    #[msg("Subscription と Plan が一致しません")]
    PlanMismatch,
}
