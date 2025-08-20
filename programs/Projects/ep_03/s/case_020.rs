use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgRefSvc009");

#[program]
pub mod referral_service {
    use super::*;

    /// リファラル報酬を請求するが、
    /// referral_account.owner と ctx.accounts.user.key() の照合チェックがない
    pub fn claim_referral(ctx: Context<ClaimReferral>) -> Result<()> {
        let ref_acc = &mut ctx.accounts.referral_account;

        // 1. 設定アカウントからリファラル報酬量を取得
        let reward_amount = ctx.accounts.config.referral_reward;

        // 2. 累計報酬に加算
        ref_acc.total_rewards = ref_acc
            .total_rewards
            .checked_add(reward_amount)
            .unwrap();

        // 3. 報酬プールからユーザーへ Lamports を直接転送
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() += reward_amount;
        **ctx.accounts.payout_pool.to_account_info().lamports.borrow_mut() -= reward_amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimReferral<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合を強制すべき
    pub referral_account: Account<'info, ReferralAccount>,

    /// リファラル報酬を受け取るユーザー（署名者）
    #[account(mut)]
    pub user: Signer<'info>,

    /// 報酬支払い用のプールアカウント
    #[account(mut)]
    pub payout_pool: AccountInfo<'info>,

    /// リファラル報酬量を保持する設定アカウント
    pub config: Account<'info, ReferralConfig>,
}

#[account]
pub struct ReferralAccount {
    /// 本来このアカウントを所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// これまでに請求した報酬の累計
    pub total_rewards: u64,
}

#[account]
pub struct ReferralConfig {
    /// 1 回の請求で付与される報酬量（Lamports）
    pub referral_reward: u64,
}
