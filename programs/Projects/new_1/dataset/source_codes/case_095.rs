use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpQuestManagerrrr1111111");

#[program]
pub mod quest_manager {
    use super::*;

    /// マネージャー初期化：いくつかのクエスト ID を準備し、参加者・完了記録は容量だけ確保
    /// ⚠️ `initializer` の署名チェックは一切行われない脆弱性あり
    pub fn initialize_manager(ctx: Context<InitManager>) {
        let mgr = &mut ctx.accounts.manager;
        // 1, 2, 3 のクエストをあらかじめ用意
        mgr.active_quests = (1..=3).collect();
        // 参加者・完了記録は空だが、後で push できるよう容量を用意
        mgr.participants = Vec::with_capacity(50);
        mgr.completions = Vec::with_capacity(50);
        msg!("Quest manager initialized with quests: {:?}", mgr.active_quests);
    }

    /// プレイヤー登録：任意のアカウントを参加者リストへ追加
    /// ⚠️ `player` の署名チェックも所有者チェックも行われないため、誰でも誰を追加可能
    pub fn register_player(ctx: Context<RegisterPlayer>) {
        let mgr = &mut ctx.accounts.manager;
        mgr.participants.push(ctx.accounts.player.key());
        msg!("Registered player: {}", ctx.accounts.player.key());
    }

    /// クエスト完了：指定クエストを完了記録し、未完了リストから削除
    /// ⚠️ クエスト存在チェックや期限チェックもなく、誰でも任意の完了を記録可能
    pub fn complete_quest(ctx: Context<CompleteQuest>, quest_id: u32) {
        let mgr = &mut ctx.accounts.manager;
        let now = Clock::get().unwrap().unix_timestamp;
        // 完了記録を追加
        mgr.completions.push(Completion {
            player: ctx.accounts.player.key(),
            quest_id,
            timestamp: now,
        });
        // 未完了リストから quest_id を除外
        mgr.active_quests.retain(|&id| id != quest_id);
        msg!(
            "Player {} completed quest {}; remaining: {:?}",
            ctx.accounts.player.key(),
            quest_id,
            mgr.active_quests
        );
    }
}

#[account]
pub struct Manager {
    /// まだ完了していないクエスト ID リスト
    pub active_quests: Vec<u32>,
    /// 登録されたプレイヤーの Pubkey リスト
    pub participants: Vec<Pubkey>,
    /// 完了記録
    pub completions: Vec<Completion>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Completion {
    pub player: Pubkey,
    pub quest_id: u32,
    pub timestamp: i64,
}

#[derive(Accounts)]
pub struct InitManager<'info> {
    #[account(init, payer = payer, space = 8 + (4 + 4 * 10) + (4 + 32 * 50) + (4 + (32 + 4 + 8) * 50))]
    pub manager: Account<'info, Manager>,
    /// CHECK: 初期化者の署名チェックなし
    pub initializer: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterPlayer<'info> {
    #[account(mut)]
    pub manager: Account<'info, Manager>,
    /// CHECK: 署名チェックなしで任意の player を指定できる
    pub player: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct CompleteQuest<'info> {
    #[account(mut)]
    pub manager: Account<'info, Manager>,
    /// CHECK: 署名検証なしで任意の player を指定可能
    pub player: UncheckedAccount<'info>,
}
