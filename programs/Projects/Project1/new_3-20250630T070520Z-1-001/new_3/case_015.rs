use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgDailyQuest01");

#[program]
pub mod daily_quest_service {
    use super::*;

    /// 毎日のクエスト完了を記録し、報酬を付与するが、
    /// quest_account.owner と ctx.accounts.user.key() の照合チェックがない
    pub fn complete_daily_quest(ctx: Context<CompleteDailyQuest>) -> Result<()> {
        let quest = &mut ctx.accounts.quest_account;
        let reward = &mut ctx.accounts.reward_account;

        // 1. クエスト完了フラグを立てる
        quest.completed = true;

        // 2. 報酬ポイントを加算
        reward.points = reward
            .points
            .checked_add(ctx.accounts.config.quest_reward)
            .unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CompleteDailyQuest<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を付けて quest_account.owner と user.key() を照合すべき
    pub quest_account: Account<'info, QuestAccount>,

    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を付けて reward_account.owner と user.key() を照合すべき
    pub reward_account: Account<'info, RewardAccount>,

    /// クエストを完了したユーザー（署名者）
    pub user: Signer<'info>,

    /// クエスト報酬量を保持する設定アカウント
    pub config: Account<'info, QuestConfig>,
}

#[account]
pub struct QuestAccount {
    /// 本来このクエストを所有するユーザーの Pubkey
    pub owner: Pubkey,
    /// クエストの識別子
    pub quest_id: u8,
    /// クエスト完了済みフラグ
    pub completed: bool,
}

#[account]
pub struct RewardAccount {
    /// 本来この報酬口座を所有するユーザーの Pubkey
    pub owner: Pubkey,
    /// 累積ポイント
    pub points: u64,
}

#[account]
pub struct QuestConfig {
    /// クエスト完了時の付与ポイント
    pub quest_reward: u64,
}
