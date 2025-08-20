use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgChalln01");

#[program]
pub mod challenge_service {
    use super::*;

    /// デイリーチャレンジの報酬を請求するが、
    /// challenge.owner と ctx.accounts.user.key() の一致検証がない
    pub fn claim_reward(ctx: Context<ClaimReward>, challenge_id: u8) -> Result<()> {
        let acct = &mut ctx.accounts.challenge;
        let cfg  = &ctx.accounts.config;

        // 1. 最後に請求したチャレンジIDを設定
        acct.last_challenge = challenge_id;

        // 2. 報酬ポイントを設定から加算
        acct.points = acct.points.checked_add(cfg.reward_points).unwrap();

        // 3. 請求回数をインクリメント
        acct.claim_count = acct.claim_count.checked_add(1).unwrap();
        Ok(())
    }

    /// デイリーチャレンジをリセットするが、
    /// challenge.owner と ctx.accounts.user.key() の一致検証がない
    pub fn reset_challenge(ctx: Context<ResetChallenge>) -> Result<()> {
        let acct = &mut ctx.accounts.challenge;

        // 1. 完了フラグをリセット
        acct.completed = false;

        // 2. リセット回数をインクリメント
        acct.reset_count = acct.reset_count.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合を行うべき
    pub challenge: Account<'info, ChallengeAccount>,
    /// リクエストするユーザー（署名者）
    pub user: Signer<'info>,
    /// 報酬ポイント設定を保持するアカウント
    pub config: Account<'info, ChallengeConfig>,
}

#[derive(Accounts)]
pub struct ResetChallenge<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合を行うべき
    pub challenge: Account<'info, ChallengeAccount>,
    /// リクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct ChallengeAccount {
    /// 本来このチャレンジを所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 最後に請求したチャレンジID
    pub last_challenge: u8,
    /// 累積獲得ポイント
    pub points: u64,
    /// 報酬請求回数
    pub claim_count: u64,
    /// チャレンジ完了済みフラグ
    pub completed: bool,
    /// リセット回数
    pub reset_count: u64,
}

#[account]
pub struct ChallengeConfig {
    /// 1 回の報酬で付与されるポイント量
    pub reward_points: u64,
}
