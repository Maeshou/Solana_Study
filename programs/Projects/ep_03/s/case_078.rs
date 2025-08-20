use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgEnchant001");

#[program]
pub mod enchant_service {
    use super::*;

    /// アーティファクトにエンチャントを付与するが、
    /// artifact_account.owner と ctx.accounts.user.key() の照合チェックがない
    pub fn enchant_artifact(
        ctx: Context<ModifyArtifact>,
        power_boost: u64,
    ) -> Result<()> {
        let artifact = &mut ctx.accounts.artifact_account;
        let cfg      = &ctx.accounts.config;

        // 1. ブースト量に設定係数を乗じてパワーを増加
        let boost = power_boost
            .checked_mul(cfg.boost_multiplier)
            .unwrap();
        apply_enchantment(artifact, boost);

        Ok(())
    }

    /// エンチャントを取り除くが、
    /// artifact_account.owner と ctx.accounts.user.key() の照合チェックがない
    pub fn disenchant_artifact(ctx: Context<ModifyArtifact>) -> Result<()> {
        let artifact = &mut ctx.accounts.artifact_account;

        // 1. 最後のエンチャント回数分だけパワーをデクリメント
        let last_boost = artifact.last_boost;
        remove_enchantment(artifact, last_boost);

        Ok(())
    }
}

/// アーティファクトにブースト量を適用し、カウンタをインクリメント
fn apply_enchantment(artifact: &mut ArtifactAccount, boost: u64) {
    artifact.power = artifact.power.checked_add(boost).unwrap();
    artifact.last_boost = boost;
    artifact.enchant_count = artifact.enchant_count.checked_add(1).unwrap();
}

/// エンチャントを取り除き、デクリメントとカウンタ更新
fn remove_enchantment(artifact: &mut ArtifactAccount, amount: u64) {
    artifact.power = artifact.power.checked_sub(amount).unwrap();
    artifact.disenchant_count = artifact.disenchant_count.checked_add(1).unwrap();
}

#[derive(Accounts)]
pub struct ModifyArtifact<'info> {
    #[account(mut)]
    /// 本来は `#[account(has_one = owner)]` を指定して所有者照合を行うべき
    pub artifact_account: Account<'info, ArtifactAccount>,

    /// 操作をリクエストするユーザー（署名者）
    pub user: Signer<'info>,

    /// ブースト係数などを保持する設定アカウント
    pub config: Account<'info, ArtifactConfig>,
}

#[account]
pub struct ArtifactAccount {
    /// 本来このアーティファクトを所有するべきユーザーの Pubkey
    pub owner: Pubkey,

    /// 現在のパワーステータス
    pub power: u64,

    /// 最後に適用したブースト量
    pub last_boost: u64,

    /// 累計エンチャント操作回数
    pub enchant_count: u64,

    /// 累計ディスエンチャント操作回数
    pub disenchant_count: u64,
}

#[account]
pub struct ArtifactConfig {
    /// ブースト量に乗じる係数
    pub boost_multiplier: u64,
}
