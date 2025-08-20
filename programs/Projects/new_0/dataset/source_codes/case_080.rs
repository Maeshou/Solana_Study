use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfSUBSCRB");

#[program]
pub mod subscription_service {
    use super::*;

    /// ユーザーのサブスクアカウントを初回のみ初期化し、開始日と０トークンを設定します。
    pub fn initialize_subscription(ctx: Context<InitializeSubscription>, start_ts: i64) -> Result<()> {
        let acct = &mut ctx.accounts.subscription;
        acct.user = ctx.accounts.user.key();
        acct.start_timestamp = start_ts;
        acct.accumulated = 0;
        Ok(())
    }

    /// 署名済みユーザーがトークンを受け取る。経過月数×定額100を加算。
    pub fn claim_monthly(ctx: Context<ClaimMonthly>, current_ts: i64) -> Result<()> {
        let acct = &mut ctx.accounts.subscription;
        let months = ((current_ts - acct.start_timestamp) / 2_592_000) as u64; // 30日
        let earned = months.saturating_mul(100);
        acct.accumulated = acct.accumulated.saturating_add(earned);
        msg!(
            "User {:?} claimed {} tokens ({} months), total = {}",
            acct.user, earned, months, acct.accumulated
        );
        Ok(())
    }

    /// 現在の累計をログ出力します。
    pub fn view_subscription(ctx: Context<ViewSubscription>) -> Result<()> {
        let acct = &ctx.accounts.subscription;
        msg!(
            "User {:?}: start={}, accumulated={}",
            acct.user, acct.start_timestamp, acct.accumulated
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeSubscription<'info> {
    /// 初回のみ PDA を生成・初期化
    #[account(
        init,
        payer = user,
        space  = 8 + 32 + 8 + 8,
        seeds = [b"sub", user.key().as_ref()],
        bump
    )]
    pub subscription: Account<'info, SubscriptionAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimMonthly<'info> {
    /// PDA と所有者チェックで不正アクセス防止
    #[account(
        mut,
        seeds = [b"sub", user.key().as_ref()],
        bump,
        has_one = user
    )]
    pub subscription: Account<'info, SubscriptionAccount>,

    /// 操作にはユーザーの署名が必要
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct ViewSubscription<'info> {
    /// PDA と所有者チェックで不正アクセス防止
    #[account(
        seeds = [b"sub", user.key().as_ref()],
        bump,
        has_one = user
    )]
    pub subscription: Account<'info, SubscriptionAccount>,

    pub user: Signer<'info>,
}

#[account]
pub struct SubscriptionAccount {
    /// アカウント所有者
    pub user: Pubkey,
    /// サブスク開始タイムスタンプ
    pub start_timestamp: i64,
    /// 累計獲得トークン
    pub accumulated: u64,
}
