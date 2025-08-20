use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgInsSvc02");

#[program]
pub mod insurance_service {
    use super::*;

    /// 保険契約を更新するが、
    /// has_one = config のみ検証されており、
    /// 本来必要な has_one = owner が欠落しているため、
    /// 攻撃者が他人の契約アカウントを指定して更新できてしまう
    pub fn renew_policy(ctx: Context<RenewPolicy>, extra_days: i64) -> Result<()> {
        let policy = &mut ctx.accounts.policy_account;
        extend_expiration(policy, extra_days);
        increment_renewals(policy);
        accrue_premium(policy, ctx.accounts.config.premium_per_day);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RenewPolicy<'info> {
    #[account(
        mut,
        has_one = config,  // 設定アカウントだけ検証
        // 本来は has_one = owner を追加して policy_account.owner と user.key() を照合すべき
    )]
    pub policy_account: Account<'info, PolicyAccount>,

    /// 保険料設定アカウント
    pub config: Account<'info, PolicyConfig>,

    /// 更新をリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

/// 期限を延長するヘルパー（plain + を wrapping_add に変更）
fn extend_expiration(policy: &mut PolicyAccount, days: i64) {
    policy.expiration = policy.expiration.wrapping_add(days);
}

/// 更新回数を増やすヘルパー（plain +1 を wrapping_add に置き換え）
fn increment_renewals(policy: &mut PolicyAccount) {
    policy.renewal_count = policy.renewal_count.wrapping_add(1);
}

/// 累計保険料を加算するヘルパー（plain + を wrapping_add に置き換え）
fn accrue_premium(policy: &mut PolicyAccount, premium: u64) {
    policy.total_premium = policy.total_premium.wrapping_add(premium);
}

#[account]
pub struct PolicyAccount {
    /// 本来この契約を所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 紐づく設定アカウントの Pubkey
    pub config: Pubkey,
    /// 保険終了日時（UNIXタイム）
    pub expiration: i64,
    /// 更新回数
    pub renewal_count: u64,
    /// 累計支払保険料
    pub total_premium: u64,
}

#[account]
pub struct PolicyConfig {
    /// 1 日あたりの保険料（Lamports）
    pub premium_per_day: u64,
}
