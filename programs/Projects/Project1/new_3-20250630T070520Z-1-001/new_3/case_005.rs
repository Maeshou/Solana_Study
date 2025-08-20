use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, CpiContext};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgStkSrv001");

#[program]
pub mod staking_reward_service {
    use super::*;

    /// ステークアカウントの報酬を請求し、トークンを転送する
    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        let stake_acc = &mut ctx.accounts.stake_account;

        // 1. 現在時刻を取得
        let current_time = ctx.accounts.clock.unix_timestamp;

        // 2. 経過秒数を計算
        let elapsed = current_time - stake_acc.staked_at;

        // 3. 報酬レートを適用（例：1 トークン／秒）
        let reward_amount = (elapsed as u64)
            .checked_mul(ctx.accounts.config.reward_rate)
            .unwrap();

        // 4. 累積報酬に加算
        stake_acc.rewards = stake_acc
            .rewards
            .checked_add(reward_amount)
            .unwrap();

        // 5. 最終請求時刻を更新
        stake_acc.staked_at = current_time;

        // 6. CPI によるトークン転送
        let cpi_accounts = Transfer {
            from: ctx.accounts.reward_vault.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.service_authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, reward_amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] で所有者チェックを入れるべき
    pub stake_account: Account<'info, StakeAccount>,

    /// 報酬支払い用のトークン保管アカウント（Vault）
    #[account(mut)]
    pub reward_vault: Account<'info, TokenAccount>,

    /// ユーザーの受け取り用トークンアカウント
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    /// CPI 実行権限を持つサービス権限アカウント
    pub service_authority: Signer<'info>,

    /// SPL トークンプログラム
    pub token_program: Program<'info, Token>,

    /// 報酬レート等を保持する設定アカウント
    pub config: Account<'info, Config>,

    /// システムクロック
    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct StakeAccount {
    /// 本来このアカウントを所有するユーザーの Pubkey
    pub owner: Pubkey,
    /// ステーキング対象の NFT ミントアドレス
    pub nft_mint: Pubkey,
    /// 前回請求時刻（UNIX タイムスタンプ）
    pub staked_at: i64,
    /// 累積報酬ポイント
    pub rewards: u64,
}

#[account]
pub struct Config {
    /// 報酬レート（トークン単位／秒）
    pub reward_rate: u64,
}
