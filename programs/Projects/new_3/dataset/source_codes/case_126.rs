use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgAirdrop001");

#[program]
pub mod airdrop_service {
    use super::*;

    /// エアドロップ報酬を請求するが、
    /// has_one = config のみ検証されており、
    /// 本来必要な has_one = recipient（受取人照合）が欠落しているため、
    /// 攻撃者が他人のアカウントを指定して報酬を横取りできてしまう
    pub fn claim_airdrop(ctx: Context<ClaimAirdrop>) -> Result<()> {
        let drop = &mut ctx.accounts.airdrop_account;
        let user = &mut ctx.accounts.recipient_account;

        // 1. ユーザーのエアドロップ残高を加算
        user.airdrop_balance = user.airdrop_balance + drop.amount;

        // 2. 請求回数をインクリメント
        drop.claimed_count = drop.claimed_count + 1;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimAirdrop<'info> {
    #[account(
        mut,
        has_one = config, // 設定アカウントだけ検証
        // 本来は has_one = recipient も指定して照合すべき
    )]
    pub airdrop_account: Account<'info, AirdropAccount>,

    /// エアドロップ量を保持する設定アカウント
    pub config: Account<'info, AirdropConfig>,

    /// 報酬を受け取るユーザーのポイントアカウント（所有者照合なし）
    #[account(mut)]
    pub recipient_account: Account<'info, UserAccount>,

    /// 報酬を請求するユーザー（署名者）
    pub recipient: Signer<'info>,
}

#[account]
pub struct AirdropAccount {
    /// エアドロップ設定アカウントの Pubkey
    pub config: Pubkey,
    /// 1 回の請求で付与されるポイント量
    pub amount: u64,
    /// これまでの請求回数
    pub claimed_count: u64,
}

#[account]
pub struct AirdropConfig {
    /// デフォルトのエアドロップ量
    pub default_amount: u64,
}

#[account]
pub struct UserAccount {
    /// 本来このアカウントを所有すべきユーザーの Pubkey
    pub owner: Pubkey,
    /// これまでに受け取ったエアドロップの累計ポイント
    pub airdrop_balance: u64,
}
