use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("SubS111111111111111111111111111111111111");

#[program]
pub mod subscription_manager {
    /// 新規サブスクリプション作成
    pub fn create_subscription(
        ctx: Context<CreateSubscription>,
        period_days: u32,
    ) -> Result<()> {
        // 期間は必ず1日以上
        if period_days == 0 {
            return Err(ErrorCode::InvalidPeriod.into());
        }

        let sub = &mut ctx.accounts.subscription;
        sub.owner  = ctx.accounts.user.key();                  // Signer Authorization
        let now     = ctx.accounts.clock.unix_timestamp;
        // 有効期限 = 現在時刻 + period_days 日
        sub.expiry = now + (period_days as i64) * 86_400;
        Ok(())
    }

    /// サブスクリプション延長
    pub fn renew_subscription(
        ctx: Context<ModifySubscription>,
        period_days: u32,
    ) -> Result<()> {
        // 期間は必ず1日以上
        if period_days == 0 {
            return Err(ErrorCode::InvalidPeriod.into());
        }

        let sub = &mut ctx.accounts.subscription;
        // 所有者チェック
        if sub.owner != ctx.accounts.user.key() {
            return Err(ErrorCode::Unauthorized.into());
        }
        let now = ctx.accounts.clock.unix_timestamp;
        sub.expiry = now + (period_days as i64) * 86_400;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateSubscription<'info> {
    /// init 制約で同一アカウント再初期化を防止（Reinit Attack）
    #[account(init, payer = user, space = 8 + 32 + 8)]
    pub subscription: Account<'info, Subscription>,

    /// サブスクライバー（署名必須）
    #[account(mut)]
    pub user:         Signer<'info>,

    /// 現在時刻取得用
    pub clock:        Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifySubscription<'info> {
    /// Account<> による Owner Check & Type Cosplay
    #[account(mut)]
    pub subscription: Account<'info, Subscription>,

    /// サブスクライバー（署名必須）
    pub user:         Signer<'info>,

    /// 現在時刻取得用
    pub clock:        Sysvar<'info, Clock>,
}

#[account]
pub struct Subscription {
    /// このサブスクリプションを操作できるユーザー
    pub owner:  Pubkey,
    /// 有効期限の UNIX タイムスタンプ
    pub expiry: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("権限がありません")]
    Unauthorized,
    #[msg("期間は1日以上で指定してください")]
    InvalidPeriod,
}
