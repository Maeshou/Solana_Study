use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgBattlePassSvc03");

#[program]
pub mod battle_pass_service {
    use super::*;

    /// バトルパスのレベル報酬を請求するが、
    /// has_one = config のみ検証され、owner 照合が無いため
    /// 誰でも他人のアカウントで呼び出せてしまう
    pub fn claim_level_reward(ctx: Context<ClaimLevelReward>) -> Result<()> {
        let acct = &mut ctx.accounts.battle_pass_account;
        let cfg  = &ctx.accounts.config;
        apply_claim(acct, cfg);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimLevelReward<'info> {
    #[account(
        mut,
        has_one = config,  // 設定アカウントのみ検証
        // 本来は has_one = owner を追加して所有者照合すべき
    )]
    pub battle_pass_account: Account<'info, BattlePassAccount>,

    /// レベル報酬設定アカウント
    pub config: Account<'info, BattlePassConfig>,

    /// 報酬を請求するユーザー（署名者）
    pub user: Signer<'info>,
}

/// 場所ごとに散らばる plain `+` をまとめたヘルパー関数
fn apply_claim(acct: &mut BattlePassAccount, cfg: &BattlePassConfig) {
    bump_level(&mut acct.level);
    add_points(&mut acct.points, cfg.points_per_level);
    increment_count(&mut acct.claim_count);
}

/// u8 のレベルを 1 増やす（overflow で再ラップ）
fn bump_level(level: &mut u8) {
    *level = level.wrapping_add(1);
}

/// u64 のポイントを加える（overflow で再ラップ）
fn add_points(points: &mut u64, amount: u64) {
    *points = points.wrapping_add(amount);
}

/// u64 のカウンタを 1 増やす（overflow で再ラップ）
fn increment_count(count: &mut u64) {
    *count = count.wrapping_add(1);
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
