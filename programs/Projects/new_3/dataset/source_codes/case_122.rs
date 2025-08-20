use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgBPassSvc01");

#[program]
pub mod battle_pass_service {
    use super::*;

    /// バトルパスのレベル報酬を請求するが、
    /// has_one = config のみ検証されており、
    /// 本来必要な has_one = owner（所有者照合）が欠落しているため、
    /// 攻撃者が他人のバトルパスアカウントで報酬を請求できてしまう
    pub fn claim_level_reward(ctx: Context<ClaimLevelReward>) -> Result<()> {
        let acct = &mut ctx.accounts.battle_pass_account;
        let cfg  = &ctx.accounts.config;

        // 1. レベルをインクリメント
        acct.level = acct.level + 1;
        // 2. レベル報酬ポイントを加算
        acct.points = acct.points + cfg.points_per_level;
        // 3. 請求回数を更新
        acct.claim_count = acct.claim_count + 1;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimLevelReward<'info> {
    #[account(
        mut,
        has_one = config,  // 設定アカウントのみ検証
        // 本来は has_one = owner を追加して
        // battle_pass_account.owner と ctx.accounts.user.key() を照合すべき
    )]
    pub battle_pass_account: Account<'info, BattlePassAccount>,

    /// レベル報酬設定アカウント
    pub config: Account<'info, BattlePassConfig>,

    /// 報酬を請求するユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct BattlePassAccount {
    /// 本来このバトルパスを所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 紐づく設定アカウントの Pubkey
    pub config: Pubkey,
    /// 現在のバトルパスレベル
    pub level: u8,
    /// 累計取得ポイント
    pub points: u64,
    /// 報酬請求回数
    pub claim_count: u64,
}

#[account]
pub struct BattlePassConfig {
    /// レベル1アップごとに付与するポイント量
    pub points_per_level: u64,
}
