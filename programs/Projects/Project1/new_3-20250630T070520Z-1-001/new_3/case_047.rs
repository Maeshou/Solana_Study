use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgLvlSvc001");

#[program]
pub mod level_service {
    use super::*;

    /// プレイヤーがゲームレベルをクリアし、報酬を受け取るが、
    /// LevelProgress.owner と ctx.accounts.user.key() の照合チェックがない
    pub fn complete_level(ctx: Context<CompleteLevel>, level: u8) -> Result<()> {
        let progress = &mut ctx.accounts.progress;

        // ↓ 本来は #[account(has_one = owner)] を指定して所有者一致を検証すべき
        progress.current_level = level;
        progress.completions = progress.completions.checked_add(1).unwrap();

        // レベルクリア報酬をプールからユーザーへ直接転送
        let reward = ctx.accounts.config.level_reward;
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() += reward;
        **ctx.accounts.reward_pool.to_account_info().lamports.borrow_mut() -= reward;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CompleteLevel<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合を行うべき
    pub progress: Account<'info, LevelProgress>,

    /// 報酬を受け取るユーザー（署名者）
    #[account(mut)]
    pub user: Signer<'info>,

    /// 報酬資金を保有するプールアカウント
    #[account(mut)]
    pub reward_pool: AccountInfo<'info>,

    /// レベルクリアごとの報酬量を保持する設定アカウント
    pub config: Account<'info, LevelConfig>,
}

#[account]
pub struct LevelProgress {
    /// 本来この進捗を所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 現在クリアしたレベル
    pub current_level: u8,
    /// これまでにクリアした回数
    pub completions: u64,
}

#[account]
pub struct LevelConfig {
    /// 1 レベルクリアごとに付与される Lamports 数
    pub level_reward: u64,
}
