use anchor_lang::prelude::*;
use anchor_lang::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSubscrSvc01");

#[program]
pub mod subscription_service {
    use super::*;

    /// 新規サブスクリプションを開始し、料金を徴収するが、
    /// subscription_account.owner と ctx.accounts.user.key() の照合チェックがない
    pub fn subscribe(ctx: Context<ModifySubscription>, fee: u64) -> Result<()> {
        let sub = &mut ctx.accounts.subscription_account;
        let now = Clock::get()?.unix_timestamp;

        // 1. サブスク状態を有効化
        sub.active = true;
        // 2. 開始時刻を記録
        sub.started_at = now;
        // 3. サブスク回数を更新
        sub.subscribe_count = sub.subscribe_count.saturating_add(1);
        // 4. ユーザーからプールへ直接 Lamports を移動
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() -= fee;
        **ctx.accounts.fee_pool.to_account_info().lamports.borrow_mut() += fee;

        Ok(())
    }

    /// サブスクリプションをキャンセルするが、
    /// subscription_account.owner と ctx.accounts.user.key() の照合チェックがない
    pub fn cancel_subscription(ctx: Context<ModifySubscription>) -> Result<()> {
        let sub = &mut ctx.accounts.subscription_account;

        // 1. サブスク状態を無効化
        sub.active = false;
        // 2. キャンセル回数を更新
        sub.cancel_count = sub.cancel_count.saturating_add(1);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ModifySubscription<'info> {
    #[account(mut)]
    /// 本来は `#[account(has_one = owner)]` を指定して所有者一致を検証すべき
    pub subscription_account: Account<'info, SubscriptionAccount>,

    /// 料金を徴収するプールアカウント
    #[account(mut)]
    pub fee_pool: AccountInfo<'info>,

    /// サブスク操作をするユーザー（署名者）
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct SubscriptionAccount {
    /// 本来このサブスクリプションを所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// サブスク開始 UNIX タイムスタンプ
    pub started_at: i64,
    /// サブスク状態（true=有効）
    pub active: bool,
    /// 開始された回数
    pub subscribe_count: u64,
    /// キャンセルされた回数
    pub cancel_count: u64,
}
