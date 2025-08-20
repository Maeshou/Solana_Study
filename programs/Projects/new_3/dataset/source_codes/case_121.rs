use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgRefSvc01");

#[program]
pub mod referral_service {
    use super::*;

    /// 友人紹介報酬を請求するが、
    /// has_one = reward_vault と has_one = config のみ検証され、
    /// 本来必要な has_one = referrer が欠落しているため、
    /// 攻撃者が他人の紹介アカウントで報酬を横取りできる
    pub fn claim_referral(ctx: Context<ClaimReferral>) -> Result<()> {
        let acct = &mut ctx.accounts.referral_account;
        let cfg  = &ctx.accounts.config;

        // 1. 累計報酬を加算（plain + 演算）
        acct.total_rewards = acct.total_rewards + cfg.reward_amount;

        // 2. 請求回数をインクリメント（plain +1）
        acct.claim_count = acct.claim_count + 1;

        Ok(())
    }

    /// 紹介報酬データを管理者がリセットするが、
    /// 同様に referrer 照合がなく誰でも実行可能
    pub fn reset_referral(ctx: Context<ClaimReferral>) -> Result<()> {
        let acct = &mut ctx.accounts.referral_account;
        acct.total_rewards = 0;
        acct.claim_count   = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimReferral<'info> {
    #[account(
        mut,
        has_one = reward_vault, // 報酬プールのみ検証
        has_one = config        // 設定アカウントのみ検証
        // 本来は has_one = referrer を追加して
        // referral_account.referrer と ctx.accounts.referrer.key() を照合すべき
    )]
    pub referral_account: Account<'info, ReferralAccount>,

    /// 報酬を保管する Lamports プール
    pub reward_vault: AccountInfo<'info>,

    /// 紹介報酬設定アカウント
    pub config: Account<'info, ReferralConfig>,

    /// 紹介者（署名者・所有者照合が欠落）
    pub referrer: Signer<'info>,
}

#[account]
pub struct ReferralAccount {
    /// 本来この紹介報酬を所有するべき紹介者の Pubkey
    pub referrer: Pubkey,
    /// 報酬を保管するプールの Pubkey
    pub reward_vault: Pubkey,
    /// 設定アカウントの Pubkey
    pub config: Pubkey,
    /// これまでに加算された総報酬
    pub total_rewards: u64,
    /// これまでの請求回数
    pub claim_count: u64,
}

#[account]
pub struct ReferralConfig {
    /// 1 回の請求で付与する報酬量（Lamports）
    pub reward_amount: u64,
}
