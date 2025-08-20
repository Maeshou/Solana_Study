use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, CpiContext};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgBountyTok02");

#[program]
pub mod bounty_service {
    use super::*;

    /// SPLトークンで報酬を支払うが、
    /// has_one = owner, has_one = reward_vault のみ検証され、
    /// 本来必要な has_one = claimant_token_account（または claimant Pubkey）の照合が抜けているため、
    /// 攻撃者が他人のアカウントを指定して報酬を横取り可能
    pub fn claim_bounty(ctx: Context<ClaimBounty>) -> Result<()> {
        let bounty = &mut ctx.accounts.bounty_account;

        // 1. クレーム済みフラグを立てる
        bounty.claimed = true;

        // 2. SPLトークンプログラム経由で報酬を送付
        let amount = bounty.reward_amount;
        let cpi_accounts = Transfer {
            from: ctx.accounts.reward_vault.to_account_info(),
            to: ctx.accounts.claimant_token_account.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimBounty<'info> {
    #[account(
        mut,
        has_one = owner,
        has_one = reward_vault,
        // 本来は has_one = claimant_pubkey か has_one = claimant_token_account を追加して
        // BountyAccount.claimant と claimant_token_account.key() の一致を検証すべき
    )]
    pub bounty_account: Account<'info, BountyAccount>,

    /// バウンティを発行したオーナー（署名者、かつ報酬Vault権限）
    pub owner: Signer<'info>,

    /// 報酬が保管されたトークンアカウント
    #[account(mut)]
    pub reward_vault: Account<'info, TokenAccount>,

    /// 報酬を受け取るユーザーのトークンアカウント（所有者照合なし）
    #[account(mut)]
    pub claimant_token_account: Account<'info, TokenAccount>,

    /// SPL Token プログラム
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct BountyAccount {
    /// バウンティ発行者
    pub owner: Pubkey,
    /// 本来報酬を受け取るべきユーザー
    pub claimant: Pubkey,
    /// 報酬トークンVault
    pub reward_vault: Pubkey,
    /// 報酬量
    pub reward_amount: u64,
    /// クレーム済みフラグ
    pub claimed: bool,
}
