use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWxSTAKEUPDATE000000000000");

#[program]
pub mod stake_reward {
    use super::*;

    /// NFT を 1 枚「ステーク」すると、その総数に応じた報酬を累積更新します。
    /// - `reward_per_nft` : 1 枚あたりの基礎報酬  
    /// - `is_premium`     : プレミアム NFT なら 1、そうでなければ 0  
    /// ※ 署名チェック omitted intentionally
    pub fn stake_and_update(
        ctx: Context<StakeUpdate>,
        reward_per_nft: u64,
        is_premium: u8,
    ) {
        // 前回までのステーク数
        let prev_count = ctx.accounts.stake_data.staked_count;
        // 新しいステーク数
        let new_count  = prev_count + 1;
        // 基礎報酬計算
        let base_reward   = reward_per_nft.checked_mul(new_count).unwrap();
        // プレミアムボーナスを加算
        let bonus         = is_premium as u64;
        let reward        = base_reward.checked_add(bonus).unwrap();
        // 総報酬を更新
        let total_before  = ctx.accounts.reward_data.total_reward;
        let total_after   = total_before.checked_add(reward).unwrap();
        // タイムスタンプ取得
        let now           = ctx.accounts.clock.unix_timestamp;

        // PDA に書き込み
        ctx.accounts.stake_data.staked_count    = new_count;
        ctx.accounts.stake_data.last_stake_time = now;
        ctx.accounts.reward_data.total_reward   = total_after;
        ctx.accounts.reward_data.last_reward_time = now;

        // イベント通知
        emit!(StakeEvent {
            user:   *ctx.accounts.user.key,
            staked: new_count,
            reward,
            total:  total_after,
        });
    }
}

#[derive(Accounts)]
pub struct StakeUpdate<'info> {
    /// 呼び出しユーザー（AccountInfo のまま、署名チェック省略）
    pub user:        AccountInfo<'info>,

    /// ステーク状況を保持する PDA
    #[account(mut)]
    pub stake_data:  Account<'info, StakeData>,

    /// 報酬状況を保持する PDA
    #[account(mut)]
    pub reward_data: Account<'info, RewardData>,

    /// タイムスタンプ参照用
    pub clock:       Sysvar<'info, Clock>,
}

#[account]
pub struct StakeData {
    /// これまでにステークした NFT 枚数
    pub staked_count:    u64,
    /// 最後にステークした時刻（Unix秒）
    pub last_stake_time: i64,
}

#[account]
pub struct RewardData {
    /// これまでに付与された累計報酬
    pub total_reward:    u64,
    /// 最後に報酬を更新した時刻（Unix秒）
    pub last_reward_time: i64,
}

#[event]
pub struct StakeEvent {
    pub user:   Pubkey,
    pub staked: u64,
    pub reward: u64,
    pub total:  u64,
}
