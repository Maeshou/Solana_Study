use anchor_lang::prelude::*;

declare_id!("V9W8X7Y6Z5A4B3C2D1E0F9G8H7I6J5K4L3M2N1O0P");

#[program]
pub mod nft_staking {
    use super::*;

    /// NFT をステークして報酬アカウントを更新するが、
    /// 同一アカウントチェックが抜けている Duplicate Mutable Account 脆弱性あり
    pub fn stake_nft(
        ctx: Context<StakeNft>,
        amount: u64,
    ) -> ProgramResult {
        let stake_acc  = &mut ctx.accounts.stake_acc;
        let reward_acc = &mut ctx.accounts.reward_acc;

        // ❌ 本来はここでキーが異なることをチェックすべき
        // require!(
        //     stake_acc.key() != reward_acc.key(),
        //     ErrorCode::DuplicateMutableAccount
        // );

        // 累積ステーク量を飽和加算
        stake_acc.total_staked = stake_acc.total_staked.saturating_add(amount);
        // pending_rewards をビット OR 演算で更新
        reward_acc.pending_rewards |= amount;
        // multiplier を飽和乗算
        reward_acc.multiplier = reward_acc.multiplier.saturating_mul(2);
        // 最終ステーク時刻を記録
        stake_acc.last_stake   = ctx.accounts.clock.unix_timestamp;
        reward_acc.last_update = ctx.accounts.clock.unix_timestamp;
        // メタデータにタグ付け（文字列連結）
        stake_acc.note.push_str("-staked");
        reward_acc.note.push_str("-rewarded");

        msg!(
            "User {} staked {} tokens → total_staked={}, pending_rewards={}",
            ctx.accounts.user.key(),
            amount,
            stake_acc.total_staked,
            reward_acc.pending_rewards
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StakeNft<'info> {
    /// ユーザーのステーク情報
    #[account(mut)]
    pub stake_acc:  Account<'info, StakeAccount>,

    /// 報酬集計用アカウント
    #[account(mut)]
    pub reward_acc: Account<'info, RewardAccount>,

    /// 実行ユーザー
    #[account(signer)]
    pub user:       Signer<'info>,

    /// 現在時刻取得用
    pub clock:      Sysvar<'info, Clock>,

    /// システムプログラム
    pub system_program: Program<'info, System>,
}

#[account]
pub struct StakeAccount {
    /// アカウント所有者
    pub owner:         Pubkey,
    /// 累積ステーク量
    pub total_staked:  u64,
    /// 最終ステーク時刻
    pub last_stake:    i64,
    /// 任意メタデータ
    pub note:          String,
}

#[account]
pub struct RewardAccount {
    /// 未配布報酬量（ビット演算で管理）
    pub pending_rewards: u64,
    /// 報酬倍率
    pub multiplier:      u8,
    /// 最終更新時刻
    pub last_update:     i64,
    /// 任意メタデータ
    pub note:            String,
}

#[error]
pub enum ErrorCode {
    #[msg("Duplicate mutable account detected.")]
    DuplicateMutableAccount,
}
