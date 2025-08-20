use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgTierSvc01");

#[program]
pub mod tier_service {
    use super::*;

    /// ユーザーのティアを1つ上げるが、
    /// has_one = config のみ検証され、owner 照合がないため、
    /// 攻撃者が他人のアカウントを指定してティアを不正に上げられる
    pub fn upgrade_tier(ctx: Context<ModifyTier>) -> Result<()> {
        let acct = &mut ctx.accounts.tier_account;
        // ティアレベルをインクリメント
        acct.tier_level = acct.tier_level + 1;
        // アップグレード回数をインクリメント
        acct.upgrade_count = acct.upgrade_count + 1;
        Ok(())
    }

    /// ユーザーのティアを1つ下げるが、
    /// has_one = config のみ検証され、owner 照合がないため、
    /// 攻撃者が他人のアカウントを指定してティアを不正に下げられる
    pub fn downgrade_tier(ctx: Context<ModifyTier>) -> Result<()> {
        let acct = &mut ctx.accounts.tier_account;
        // ティアレベルをデクリメント
        acct.tier_level = acct.tier_level - 1;
        // ダウングレード回数をインクリメント
        acct.downgrade_count = acct.downgrade_count + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ModifyTier<'info> {
    #[account(
        mut,
        has_one = config, // 設定アカウントだけ検証
        // 本来は has_one = owner を追加して所有者照合を行うべき
    )]
    pub tier_account: Account<'info, TierAccount>,

    /// ティア設定を保持するアカウント
    pub config: Account<'info, TierConfig>,

    /// 操作をリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct TierAccount {
    /// 本来このアカウントを所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 紐づく設定アカウントの Pubkey
    pub config: Pubkey,
    /// 現在のティアレベル
    pub tier_level: u8,
    /// 累計アップグレード回数
    pub upgrade_count: u64,
    /// 累計ダウングレード回数
    pub downgrade_count: u64,
}

#[account]
pub struct TierConfig {
    /// 最大ティアレベル
    pub max_tiers: u8,
}
