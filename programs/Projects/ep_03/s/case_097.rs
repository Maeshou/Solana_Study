use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgRewardXchg01");

#[program]
pub mod reward_exchange {
    use super::*;

    /// ユーザーのポイントを商品に交換するが、
    /// exchange_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn redeem_points(ctx: Context<ModifyExchange>, points: u64) -> Result<()> {
        let acct = &mut ctx.accounts.exchange_account;
        perform_redemption(acct, points);
        Ok(())
    }

    /// 交換をキャンセルしてポイントを返却するが、
    /// exchange_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn refund_points(ctx: Context<ModifyExchange>, points: u64) -> Result<()> {
        let acct = &mut ctx.accounts.exchange_account;
        perform_refund(acct, points);
        Ok(())
    }
}

/// ポイント交換を記録し、累計交換数を更新するヘルパー関数
fn perform_redemption(acct: &mut ExchangeAccount, points: u64) {
    // 1. 現在のポイント残高から差し引き（オーバーフロー回避に saturating_sub）
    acct.balance = acct.balance.saturating_sub(points);
    // 2. 最後に交換したポイントを記録
    acct.last_redeemed = points;
    // 3. 交換回数を更新
    acct.redeem_count = acct.redeem_count.saturating_add(1);
}

/// ポイント返却を記録し、累計返却数を更新するヘルパー関数
fn perform_refund(acct: &mut ExchangeAccount, points: u64) {
    // 1. ポイント残高に加算
    acct.balance = acct.balance.saturating_add(points);
    // 2. 最後に返却したポイントを記録
    acct.last_refunded = points;
    // 3. 返却回数を更新
    acct.refund_count = acct.refund_count.saturating_add(1);
}

#[derive(Accounts)]
pub struct ModifyExchange<'info> {
    #[account(mut)]
    /// 本来は `#[account(has_one = owner)]` を指定して所有者照合を行うべき
    pub exchange_account: Account<'info, ExchangeAccount>,
    /// 操作をリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

/// ユーザーのポイント交換を管理するアカウント
#[account]
pub struct ExchangeAccount {
    /// 本来このアカウントを所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 現在のポイント残高
    pub balance: u64,
    /// 最後に交換したポイント数
    pub last_redeemed: u64,
    /// 累計交換回数
    pub redeem_count: u64,
    /// 最後に返却したポイント数
    pub last_refunded: u64,
    /// 累計返却回数
    pub refund_count: u64,
}
