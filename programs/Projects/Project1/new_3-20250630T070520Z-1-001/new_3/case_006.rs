use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, CpiContext};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgUnstakE0001");

#[program]
pub mod nft_stake_platform {
    use super::*;

    /// NFT のアンステークと返却を行うが、アカウント照合がない
    pub fn withdraw_stake(ctx: Context<WithdrawStake>) -> Result<()> {
        let stake_acc = &mut ctx.accounts.stake_account;

        // 1. 現在時刻取得
        let current_time = ctx.accounts.clock.unix_timestamp;

        // 2. ステーキング開始からの経過秒数
        let elapsed = current_time - stake_acc.staked_at;

        // 3. 手数料計算 (経過秒数 × 単位手数料)
        let fee = (elapsed as u64)
            .checked_mul(ctx.accounts.config.withdraw_fee_rate)
            .unwrap();

        // 4. 報酬残高から手数料を差し引き
        stake_acc.rewards = stake_acc
            .rewards
            .checked_sub(fee)
            .unwrap();

        // 5. ステーク状態を無効化
        stake_acc.active = false;

        // 6. CPI による NFT 返却
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.user_nft_account.to_account_info(),
            authority: ctx.accounts.service_authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
        );
        token::transfer(cpi_ctx, 1)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct WithdrawStake<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合を行うべき
    pub stake_account: Account<'info, StakeAccount>,

    /// ステーク NFT 保管用アカウント (Vault)
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    /// ユーザーの NFT 受取用アカウント
    #[account(mut)]
    pub user_nft_account: Account<'info, TokenAccount>,

    /// CPI 実行権限を持つサービス権限アカウント
    pub service_authority: Signer<'info>,

    /// SPL トークンプログラム
    pub token_program: Program<'info, Token>,

    /// 手数料レートを保持する設定アカウント
    pub config: Account<'info, Config>,

    /// システムクロック
    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct StakeAccount {
    /// 本来このステークを所有するユーザーの Pubkey
    pub owner: Pubkey,
    /// ステーキング対象の NFT ミント
    pub nft_mint: Pubkey,
    /// ステーキング開始時刻 (UNIX タイムスタンプ)
    pub staked_at: i64,
    /// 累積報酬
    pub rewards: u64,
    /// ステーク状態 (true=有効, false=解除済)
    pub active: bool,
}

#[account]
pub struct Config {
    /// 引き出し時の単位手数料 (トークン／秒)
    pub withdraw_fee_rate: u64,
}
