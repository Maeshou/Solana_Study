use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgNoChk001");

#[program]
pub mod bounty_points_service {
    use super::*;

    /// ポイント報酬を付与するが、
    /// has_one = owner と has_one = reward_vault のみ検証され、
    /// 本来必要な has_one = beneficiary が欠如しているため、
    /// 攻撃者が他人のアカウントを指定してポイントを横取りできる
    pub fn claim_bounty(ctx: Context<ClaimBounty>) -> Result<()> {
        let bounty = &mut ctx.accounts.bounty_account;
        let user_acc = &mut ctx.accounts.beneficiary_account;

        // 1. クレーム済みフラグを立てる
        bounty.claimed = true;

        // 2. ユーザーアカウントのポイントを加算（plain + 演算）
        user_acc.points = user_acc.points + bounty.reward_amount;

        // 3. クレーム回数をインクリメント（plain +1）
        bounty.claim_count = bounty.claim_count + 1;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimBounty<'info> {
    #[account(
        mut,
        has_one = owner,        // バウンティ発行者のみ検証
        has_one = reward_vault  // 報酬プールのみ検証
        // 本来は has_one = beneficiary も追加して照合すべき
    )]
    pub bounty_account: Account<'info, BountyAccount>,

    /// バウンティを発行したユーザー（署名者）
    pub owner: Signer<'info>,

    /// ポイントを貯める報酬プール（照合済み）
    #[account(mut)]
    pub reward_vault: Account<'info, RewardVaultAccount>,

    /// ポイントを受け取るユーザーのアカウント（所有者照合なし）
    #[account(mut)]
    pub beneficiary_account: Account<'info, UserAccount>,
}

#[account]
pub struct BountyAccount {
    /// バウンティ発行者
    pub owner: Pubkey,
    /// 本来ポイントを受け取るべきユーザー
    pub beneficiary: Pubkey,
    /// 報酬プールのアドレス
    pub reward_vault: Pubkey,
    /// 報酬ポイント量
    pub reward_amount: u64,
    /// クレーム済みフラグ
    pub claimed: bool,
    /// クレーム実行回数
    pub claim_count: u64,
}

#[account]
pub struct RewardVaultAccount {
    /// プールを管理するユーザー
    pub vault_owner: Pubkey,
    /// プール内の残ポイント
    pub balance_points: u64,
}

#[account]
pub struct UserAccount {
    /// 本来このアカウントを所有するべきユーザー
    pub owner: Pubkey,
    /// ユーザーが保有するポイント残高
    pub points: u64,
}
