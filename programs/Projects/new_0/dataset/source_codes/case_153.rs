use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzT8");

#[program]
pub mod task_manager {
    use super::*;

    /// タスク作成：説明文を受け取り、完了フラグを false で初期化
    pub fn create_task(
        ctx: Context<CreateTask>,
        bump: u8,
        description: String,
    ) -> Result<()> {
        let task = &mut ctx.accounts.task;
        task.owner = ctx.accounts.user.key();
        task.bump = bump;
        task.description = description;
        task.completed = false;
        Ok(())
    }

    /// 完了状態トグル：has_one／署名チェックで所有者のみ操作可能
    pub fn toggle_task(
        ctx: Context<ToggleTask>,
    ) -> Result<()> {
        let task = &mut ctx.accounts.task;
        task.completed = !task.completed;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct CreateTask<'info> {
    /// PDA で生成する Task
    #[account(
        init,
        payer = user,
        // 8 + 32 + 1 + 4 + (最大140文字) + 1
        space = 8 + 32 + 1 + 4 + 140 + 1,
        seeds = [b"task", user.key().as_ref()],
        bump
    )]
    pub task: Account<'info, Task>,

    /// トランザクション送信者（タスク所有者）
    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ToggleTask<'info> {
    /// 既存の Task（PDA／bump 検証 + オーナーチェック）
    #[account(
        mut,
        seeds = [b"task", owner.key().as_ref()],
        bump = task.bump,
        has_one = owner
    )]
    pub task: Account<'info, Task>,

    /// Task 所有者（署名必須）
    #[account(signer)]
    pub owner: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

/// Task データ構造：オーナー、bump、説明文、完了フラグ
#[account]
pub struct Task {
    pub owner: Pubkey,
    pub bump: u8,
    pub description: String,
    pub completed: bool,
}

/// カスタムエラー（属性チェックのみで完結するため空）
#[error_code]
pub enum ErrorCode {}
