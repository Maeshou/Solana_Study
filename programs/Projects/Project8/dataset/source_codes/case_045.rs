use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_game {
    use super::*;

    // デイリークエストの報酬を受け取る
    pub fn claim_daily_quest_reward(ctx: Context<ClaimDailyQuestReward>, quest_id: u16) -> Result<()> {
        let quest_progress = &mut ctx.accounts.quest_progress;
        let player_inventory = &mut ctx.accounts.player_inventory;
        
        let mut target_quest_index = 1000; // ありえない値で初期化
        // 対象のクエストを探すループ
        for (index, quest) in quest_progress.daily_quests.iter().enumerate() {
            if quest.quest_id == quest_id {
                target_quest_index = index;
            }
        }
        require!(target_quest_index != 1000, GameError::QuestNotFound);
        
        let quest = &mut quest_progress.daily_quests[target_quest_index];

        // クエストが完了しているか、かつ報酬が未受け取りかを確認
        require!(quest.progress >= quest.target_count, GameError::QuestNotCompleted);
        require!(!quest.reward_claimed, GameError::RewardAlreadyClaimed);
        
        // 報酬をインベントリに追加
        for reward in quest.rewards.iter() {
            let mut item_found = false;
            for inventory_item in player_inventory.items.iter_mut() {
                if inventory_item.item_id == reward.item_id {
                    inventory_item.quantity += reward.quantity;
                    item_found = true;
                }
            }
            if !item_found {
                player_inventory.items.push(InventoryItem {
                    item_id: reward.item_id,
                    quantity: reward.quantity,
                });
            }
        }
        
        // 報酬受け取り済みに設定
        quest.reward_claimed = true;
        msg!("Quest {} reward claimed successfully!", quest_id);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimDailyQuestReward<'info> {
    #[account(mut, seeds = [b"quest_progress", player.key().as_ref()], bump = quest_progress.bump)]
    pub quest_progress: Account<'info, QuestProgress>,
    #[account(mut, seeds = [b"inventory", player.key().as_ref()], bump = player_inventory.bump)]
    pub player_inventory: Account<'info, PlayerInventory>,
    #[account(mut)]
    pub player: Signer<'info>,
}

#[account]
pub struct QuestProgress {
    pub daily_quests: Vec<Quest>,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Quest {
    pub quest_id: u16,
    pub progress: u32,
    pub target_count: u32,
    pub reward_claimed: bool,
    pub rewards: Vec<InventoryItem>,
}

// PlayerInventoryはパターン2のものを再利用
#[account]
pub struct PlayerInventory {
    pub items: Vec<InventoryItem>,
    pub bump: u8,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InventoryItem {
    pub item_id: u32,
    pub quantity: u64,
}


#[error_code]
pub enum GameError {
    #[msg("The specified quest was not found.")]
    QuestNotFound,
    #[msg("The quest is not yet completed.")]
    QuestNotCompleted,
    #[msg("The reward for this quest has already been claimed.")]
    RewardAlreadyClaimed,
}