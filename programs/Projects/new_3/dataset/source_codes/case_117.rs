use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgBountySvc02");

#[program]
pub mod bounty_service {
    use super::*;

    /// バウンティをクレームし報酬を受け取るが、
    /// has_one = owner と has_one = reward_vault のみ検証され、
    /// 本来必要な has_one = claimant の照合が抜けているため、
    /// 攻撃者が他人のバウンティアカウントを指定して不正に報酬を取得できる
    pub fn claim_bounty(ctx: Context<ClaimBounty>) -> Result<()> {
        let bounty = &mut ctx.accounts.bounty_account;
        // 1. クレーム済みフラグを立てる
        bounty.claimed = true;
        // 2. プールからクレームユーザーへ Lamports を直接移動
        let reward = bounty.reward_amount;
        **ctx.accounts.reward_vault.to_account_info().lamports.borrow_mut() -= reward;
        **ctx.accounts.claimant.to_account_info().lamports.borrow_mut() += reward;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimBounty<'info> {
    #[account(
        mut,
        has_one = owner,         // バウンティ作成者だけ検証
        has_one = reward_vault   // 報酬プールだけ検証
        // 本来は has_one = claimant も追加して照合すべき
    )]
    pub bounty_account: Account<'info, BountyAccount>,

    /// バウンティを発行したオーナー（署名者）
    pub owner: Signer<'info>,

    /// 報酬を保管するプールアカウント
    #[account(mut)]
    pub reward_vault: AccountInfo<'info>,

    /// 報酬を受け取るクレームユーザー（署名者）
    #[account(mut)]
    pub claimant: Signer<'info>,
}

#[account]
pub struct BountyAccount {
    /// バウンティを発行したユーザーの Pubkey
    pub owner: Pubkey,
    /// 本来報酬を受け取るべきクレームユーザーの Pubkey
    pub claimant: Pubkey,
    /// 報酬プールの Pubkey
    pub reward_vault: Pubkey,
    /// 報酬金額 (Lamports)
    pub reward_amount: u64,
    /// クレーム済みフラグ
    pub claimed: bool,
}
