use anchor_lang::prelude::*;

declare_id!("Todo111111111111111111111111111111111111");

#[program]
pub mod todo_list {
    /// タスク作成
    pub fn create_task(
        ctx: Context<CreateTask>,
        description: String,
    ) -> Result<()> {
        // 記述長チェック（オーバーフロー防止）
        require!(
            description.len() <= 128,
            ErrorCode::DescriptionTooLong
        );

        let task = &mut ctx.accounts.task;
        task.owner       = ctx.accounts.user.key();  // Signer Authorization
        task.description = description;
        task.completed   = false;
        Ok(())
    }

    /// タスク完了マーク
    pub fn complete_task(ctx: Context<ModifyTask>) -> Result<()> {
        let task = &mut ctx.accounts.task;
        // Account Matching + Signer Authorization
        require!(
            task.owner == ctx.accounts.user.key(),
            ErrorCode::Unauthorized
        );
        task.completed = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateTask<'info> {
    /// init 制約で再初期化（Reinit Attack）を防止
    #[account(init, payer = user, space = 8 + 32 + 4 + 128 + 1)]
    pub task:   Account<'info, TaskAccount>,

    /// 実際に署名したユーザー（Signer Authorization）
    #[account(mut)]
    pub user:   Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyTask<'info> {
    /// Account<> による Owner Check / Type Cosplay
    #[account(mut)]
    pub task:   Account<'info, TaskAccount>,

    /// 実際に署名したユーザー
    pub user:   Signer<'info>,
}

#[account]
pub struct TaskAccount {
    /// このタスクを操作できるユーザー
    pub owner:       Pubkey,
    /// タスク内容（最大128文字）
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
