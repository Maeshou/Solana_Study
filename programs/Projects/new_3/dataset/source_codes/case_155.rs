use anchor_lang::prelude::*;
declare_id!("SubUpdXyz1111111111111111111111111111111");

/// 継続課金プラン情報
#[account]
pub struct Plan {
    pub owner:       Pubkey, // プラン発行者
    pub fee_per_day: u64,    // 日割り料金（lamports）
}

/// ユーザーのサブスクリプション情報
#[account]
pub struct Subscription {
    pub subscriber:  Pubkey, // 購読者
    pub plan:        Pubkey, // 本来は Plan.key() と一致すべき
    pub valid_until: i64,    // UNIX タイムスタンプでの有効期限
}

#[derive(Accounts)]
pub struct InitializePlan<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8)]
    pub plan:           Account<'info, Plan>,
    #[account(mut)]
    pub owner:          Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Subscribe<'info> {
    /// Plan.owner == owner.key() は検証される
    #[account(mut, has_one = owner)]
    pub plan:           Account<'info, Plan>,

    /// Subscription.plan == plan.key() の検証がないまま生成
    #[account(init, payer = subscriber, space = 8 + 32 + 32 + 8)]
    pub subscription:   Account<'info, Subscription>,

    #[account(mut)]
    pub subscriber:     Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExtendSubscription<'info> {
    /// Subscription.subscriber == subscriber.key() は検証される
    #[account(mut, has_one = subscriber)]
    pub subscription:   Account<'info, Subscription>,

    /// plan.key() と subscription.plan の一致チェックがない
    #[account(mut)]
    pub plan:           Account<'info, Plan>,

    #[account(mut)]
    pub subscriber:     Signer<'info>,
}

#[program]
pub mod subscription_vuln {
    use super::*;

    /// プランを新規作成
    pub fn initialize_plan(
        ctx: Context<InitializePlan>,
        fee_per_day: u64
    ) -> Result<()> {
        let p = &mut ctx.accounts.plan;
        p.owner       = ctx.accounts.owner.key();
        p.fee_per_day = fee_per_day;
        Ok(())
    }

    /// ユーザーをプランに登録
    pub fn subscribe(
        ctx: Context<Subscribe>,
        duration_days: i64
    ) -> Result<()> {
        let p  = &ctx.accounts.plan;
        let s  = &mut ctx.accounts.subscription;
        s.subscriber  = ctx.accounts.subscriber.key();
        s.plan        = p.key();        
        s.valid_until = Clock::get()?.unix_timestamp + duration_days * 24 * 60 * 60;
        Ok(())
    }

    /// サブスクリプションの延長
    pub fn extend(
        ctx: Context<ExtendSubscription>,
        extra_days: i64
    ) -> Result<()> {
        let p = &ctx.accounts.plan;
        let s = &mut ctx.accounts.subscription;
        // 本来は必須：
        // require_keys_eq!(
        //     s.plan,
        //     p.key(),
        //     ErrorCode::PlanMismatch
        // );
        // がないため、任意の Plan アカウントを渡して請求・延長処理できてしまう

        // 有効期限を延長
        s.valid_until = s.valid_until.checked_add(extra_days * 24 * 60 * 60).unwrap();
        // lamports の請求処理などは省略
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Subscription が指定の Plan と一致しません")]
    PlanMismatch,
}
