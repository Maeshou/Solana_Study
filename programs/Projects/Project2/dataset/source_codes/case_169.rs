use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzUP");

#[program]
pub mod subscription_service {
    use super::*;

    /// サブスクリプション開始：初期プランを設定し、有効化＋タイムスタンプ登録
    pub fn initialize_subscription(
        ctx: Context<InitializeSubscription>,
        initial_plan: u8,
    ) -> Result<()> {
        let sub = &mut ctx.accounts.subscription;
        // アカウントはゼロクリア済 → 必要フィールドだけ代入
        sub.owner           = ctx.accounts.user.key();
        sub.bump            = *ctx.bumps.get("subscription").unwrap();
        sub.plan            = initial_plan;
        let now = ctx.accounts.clock.unix_timestamp;
        sub.start_ts        = now;
        sub.last_payment_ts = now;
        sub.active          = true;
        Ok(())
    }

    /// プラン変更：新プランを設定し、最終支払時刻を更新
    pub fn change_plan(
        ctx: Context<ModifySubscription>,
        new_plan: u8,
    ) -> Result<()> {
        let sub = &mut ctx.accounts.subscription;
        sub.plan            = new_plan;
        sub.last_payment_ts = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }

    /// 更新支払：最終支払時刻のみを更新（自動延長などに利用）
    pub fn renew_subscription(
        ctx: Context<ModifySubscription>,
    ) -> Result<()> {
        let sub = &mut ctx.accounts.subscription;
        sub.last_payment_ts = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }

    /// サブスクリプション取消：アクティブフラグを下げ、時刻を更新
    pub fn cancel_subscription(
        ctx: Context<ModifySubscription>,
    ) -> Result<()> {
        let sub = &mut ctx.accounts.subscription;
        sub.active          = false;
        sub.last_payment_ts = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(initial_plan: u8)]
pub struct InitializeSubscription<'info> {
    /// ゼロクリア後、必要フィールドだけ設定
    #[account(
        init_zeroed,
        payer = user,
        seeds = [b"subscription", user.key().as_ref()],
        bump,
        space = 8  // discriminator
              + 32 // owner
              + 1  // bump
              + 1  // plan
              + 8  // start_ts
              + 8  // last_payment_ts
              + 1  // active
    )]
    pub subscription: Account<'info, Subscription>,

    /// サブスクリプション利用者（署名必須）
    #[account(mut)]
    pub user: Signer<'info>,

    /// 時刻取得用
    pub clock: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifySubscription<'info> {
    /// 既存の Subscription（PDA検証 + オーナーチェック）
    #[account(
        mut,
        seeds = [b"subscription", owner.key().as_ref()],
        bump = subscription.bump,
        has_one = owner
    )]
    pub subscription: Account<'info, Subscription>,

    /// 認証されたサブスク所有者（署名必須）
    #[account(signer)]
    pub owner: AccountInfo<'info>,

    /// 時刻取得用
    pub clock: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Subscription {
    pub owner:           Pubkey, // 所有者
    pub bump:            u8,     // PDA bump
    pub plan:            u8,     // プラン番号
    pub start_ts:        i64,    // 開始時刻
    pub last_payment_ts: i64,    // 最終支払時刻
    pub active:          bool,   // 有効フラグ
}
