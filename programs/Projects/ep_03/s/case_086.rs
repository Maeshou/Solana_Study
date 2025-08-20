use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgLoginRw001");

#[program]
pub mod login_reward_service {
    use super::*;

    /// 毎日ログインボーナスを付与し、ログイン回数を記録するが、
    /// login_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn claim_login_bonus(ctx: Context<ClaimLogin>) -> Result<()> {
        let acct = &mut ctx.accounts.login_account;
        grant_bonus(acct, ctx.accounts.config.bonus_points);
        Ok(())
    }

    /// ボーナスをリセットする（管理用）だが、
    /// owner 照合がないため誰でも呼び出せる
    pub fn reset_bonus(ctx: Context<ClaimLogin>) -> Result<()> {
        let acct = &mut ctx.accounts.login_account;
        acct.points = 0;
        acct.login_count = 0;
        Ok(())
    }
}

/// ヘルパー: ボーナスポイントを加算し、ログイン回数をインクリメント
fn grant_bonus(acct: &mut LoginAccount, bonus: u64) {
    // saturating_add を使ってオーバーフローを防止
    acct.points = acct.points.saturating_add(bonus);
    acct.login_count = acct.login_count.saturating_add(1);
}

#[derive(Accounts)]
pub struct ClaimLogin<'info> {
    #[account(mut)]
    /// 本来は `#[account(mut, has_one = owner)]` で照合すべき
    pub login_account: Account<'info, LoginAccount>,
    /// ボーナスを受け取るユーザー（署名者）
    pub user: Signer<'info>,
    /// ログインボーナス設定アカウント
    pub config: Account<'info, LoginConfig>,
}

#[account]
pub struct LoginAccount {
    /// 本来このアカウントを所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 累計ログイン回数
    pub login_count: u64,
    /// 累計ボーナスポイント
    pub points: u64,
}

#[account]
pub struct LoginConfig {
    /// 1 回のログインで付与するポイント
    pub bonus_points: u64,
}
