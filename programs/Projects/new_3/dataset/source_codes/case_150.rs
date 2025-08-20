use anchor_lang::prelude::*;
declare_id!("ProjTask1111111111111111111111111111111111");

/// プロジェクト情報
#[account]
pub struct Project {
    pub owner:        Pubkey,  // プロジェクト管理者
    pub title:        String,  // プロジェクト名
    pub task_count:   u64,     // 登録タスク数
}

/// タスク情報
#[account]
pub struct Task {
    pub description:  String,  // タスク内容
    pub project:      Pubkey,  // 本来は Project.key() と一致すべき
    pub completed:    bool,    // 完了フラグ
}

#[derive(Accounts)]
pub struct InitializeProject<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 64 + 8)]
    pub project:       Account<'info, Project>,
    #[account(mut)]
    pub owner:         Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddTask<'info> {
    /// Project.owner == owner.key() の検証あり
    #[account(mut, has_one = owner)]
    pub project:       Account<'info, Project>,

    /// 新規タスクを作成するが project フィールドの検証はなし
    #[account(init, payer = owner, space = 8 + 4 + 128 + 32 + 1)]
    pub task:          Account<'info, Task>,

    pub owner:         Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateTask<'info> {
    /// Project.owner == owner.key() の検証あり
    #[account(mut, has_one = owner)]
    pub project:       Account<'info, Project>,

    /// Task.project == project.key() の検証がないため、
    /// 別のプロジェクトのタスクを渡しても通ってしまう
    #[account(mut)]
    pub task:          Account<'info, Task>,

    pub owner:         Signer<'info>,
}

#[program]
pub mod proj_task_vuln {
    use super::*;

    /// プロジェクトを初期化
    pub fn initialize_project(ctx: Context<InitializeProject>, title: String) -> Result<()> {
        let p = &mut ctx.accounts.project;
        p.owner      = ctx.accounts.owner.key();
        p.title      = title;
        p.task_count = 0;
        Ok(())
    }

    /// タスクを追加
    pub fn add_task(ctx: Context<AddTask>, description: String) -> Result<()> {
        let p = &mut ctx.accounts.project;
        let t = &mut ctx.accounts.task;

        // 脆弱性ポイント：
        // t.project = p.key(); と設定するだけで、
        // タスクが本当にこのプロジェクトに属しているかは検証されない
        t.description = description;
        t.project     = p.key();
        t.completed   = false;

        p.task_count = p.task_count.checked_add(1).unwrap();
        Ok(())
    }

    /// タスクを完了にマーク
    pub fn update_task(ctx: Context<UpdateTask>) -> Result<()> {
        let t = &mut ctx.accounts.task;
        // 本来は必須：
        // require_keys_eq!(
        //     t.project,
        //     ctx.accounts.project.key(),
        //     ErrorCode::ProjectMismatch
        // );
        t.completed = true;
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Task が指定の Project と一致しません")]
    ProjectMismatch,
}
