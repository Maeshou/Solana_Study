use anchor_lang::prelude::*;

declare_id!("NextCaseTask555555555555555555555555555555");

#[program]
pub mod example10 {
    use super::*;

    // タスクリスト作成（task_list にだけ init）
    pub fn create_list(ctx: Context<CreateList>) -> Result<()> {
        let tl = &mut ctx.accounts.task_list;
        tl.total = 0;
        Ok(())
    }

    // 複数タスクを完了、失敗数を集計（fail_count は init なし）
    pub fn complete_tasks(ctx: Context<CompleteTasks>, statuses: Vec<bool>) -> Result<()> {
        let tl = &mut ctx.accounts.task_list;
        let fc = &mut ctx.accounts.fail_count; // ← init なし（本来は初期化すべき）
        fc.count = 0;

        for &s in statuses.iter() {
            if s {
                tl.total += 1;
            } else {
                fc.count += 1;
            }
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateList<'info> {
    #[account(init, payer = user, space = 8 + 4)]
    pub task_list: Account<'info, TaskListData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CompleteTasks<'info> {
    #[account(mut)] pub task_list: Account<'info, TaskListData>, // ← init なし
    pub fail_count: Account<'info, FailData>,                   // ← init なし
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct TaskListData {
    pub total: u32,
}

#[account]
pub struct FailData {
    pub count: u32,
}
