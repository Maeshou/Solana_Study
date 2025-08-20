use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgQuestSv02");

#[program]
pub mod quest_tracker {
    use super::*;

    /// クエストを開始し、開始回数を記録するが、
    /// quest_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn start_quest(ctx: Context<ModifyQuest>, quest_id: u8) -> Result<()> {
        let quest = &mut ctx.accounts.quest_account;
        record_start(quest, quest_id);
        Ok(())
    }

    /// クエストを完了し、XPを付与するが、
    /// quest_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn complete_quest(ctx: Context<ModifyQuest>) -> Result<()> {
        let quest = &mut ctx.accounts.quest_account;
        let xp = get_reward_xp(&ctx.accounts.config);
        record_completion(quest, xp);
        Ok(())
    }
}

/// クエスト開始時の状態を更新するヘルパー関数
fn record_start(quest: &mut QuestAccount, quest_id: u8) {
    quest.quest_id = quest_id;
    quest.started = true;
    quest.start_count = quest.start_count.checked_add(1).unwrap();
}

/// 設定アカウントからXP報酬を取得するヘルパー関数
fn get_reward_xp(config: &Account<QuestConfig>) -> u64 {
    config.xp_reward
}

/// クエスト完了時の状態を更新するヘルパー関数
fn record_completion(quest: &mut QuestAccount, xp: u64) {
    quest.started = false;
    quest.total_xp = quest.total_xp.checked_add(xp).unwrap();
    quest.complete_count = quest.complete_count.checked_add(1).unwrap();
}

#[derive(Accounts)]
pub struct ModifyQuest<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合を行うべき
    pub quest_account: Account<'info, QuestAccount>,
    /// 操作をリクエストするユーザー（署名者）
    pub user: Signer<'info>,
    /// XP報酬量を保持する設定アカウント
    pub config: Account<'info, QuestConfig>,
}

#[account]
pub struct QuestAccount {
    /// 本来このクエストを所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 現在のクエストID
    pub quest_id: u8,
    /// クエスト中フラグ
    pub started: bool,
    /// 開始した回数
    pub start_count: u64,
    /// 完了した回数
    pub complete_count: u64,
    /// 累計XP
    pub total_xp: u64,
}

#[account]
pub struct QuestConfig {
    /// クエスト完了ごとに付与するXP量
    pub xp_reward: u64,
}
