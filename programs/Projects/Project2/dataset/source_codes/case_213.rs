use anchor_lang::prelude::*;

declare_id!("QstExa2222222222222222222222222222222222");

#[program]
pub mod quest_extra {
    use super::*;

    pub fn attempt(ctx: Context<Attempt>, success: bool) -> Result<()> {
        let q = &mut ctx.accounts.quest;
        if success {
            // 完了フラグ
            q.completed = true;
            // 報酬支払いフラグ
            q.reward_given = true;
            // 成功回数を記録
            q.success_count = q.success_count.saturating_add(1);
        } else {
            // リトライカウント
            q.retries = q.retries.saturating_add(1);
            // ロックアウト状態を更新
            if q.retries > 3 {
                q.locked = true;
            }
            // 失敗理由コードをセット
            q.last_error = 1;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Attempt<'info> {
    #[account(mut)]
    pub quest: Account<'info, QuestExtraData>,
    pub user: Signer<'info>,
}

#[account]
pub struct QuestExtraData {
    pub completed: bool,
    pub reward_given: bool,
    pub success_count: u64,
    pub retries: u64,
    pub locked: bool,
    pub last_error: u8,
}
