use anchor_lang::prelude::*;

declare_id!("TAsk111111111111111111111111111111111111");

#[program]
pub mod task_assignment {
    /// マネージャーがタスクを新規作成
    pub fn create_task(
        ctx: Context<CreateTask>,
        description: String,
        assignee: Pubkey,
    ) -> Result<()> {
        // 説明文長チェック
        if description.len() > 256 {
            return Err(ErrorCode::DescriptionTooLong.into());
        }

        let task = &mut ctx.accounts.task;
        task.manager     = ctx.accounts.manager.key();
        task.assignee    = assignee;
        task.description = description;
        task.completed   = false;
        Ok(())
    }

    /// アサイニーがタスクを完了マーク
    pub fn complete_task(ctx: Context<CompleteTask>) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let user = ctx.accounts.user.key();
        // 所有者チェック (Assignee Authorization)
        if task.assignee != user {
            return Err(ErrorCode::Unauthorized.into());
        }
        task.completed = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateTask<'info> {
    /// 同一アカウント再初期化防止 (Reinit Attack)
    #[account(init, payer = manager, space = 8 + 32 + 32 + 4 + 256 + 1)]
    pub task:           Account<'info, TaskAccount>,

    /// 作成者（Signer Authorization）
    #[account(mut)]
    pub manager:        Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CompleteTask<'info> {
    /// 型チェック＆所有者チェック (Owner Check / Type Cosplay)
    #[account(mut)]
    pub task:           Account<'info, TaskAccount>,

    /// 実際に署名したアサイニー (Signer Authorization)
    pub user:           Signer<'info>,
}

#[account]
pub struct TaskAccount {
    /// タスクの作成者
    pub manager:     Pubkey,
    /// タスクの担当者
    pub assignee:    Pubkey,
    /// タスク内容（最大256文字）
    pub description: String,
    /// 完了フラグ
    pub completed:   bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Description is too long")]
    DescriptionTooLong,
}
