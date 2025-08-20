use anchor_lang::prelude::*;

declare_id!("MixInitMissLoop555555555555555555555555555");

#[program]
pub mod example5 {
    use super::*;

    // タスクリストを初期化（task_list にだけ init）
    pub fn init_list(ctx: Context<InitList>) -> Result<()> {
        let list = &mut ctx.accounts.task_list;
        list.count = 0;
        Ok(())
    }

    // 複数タスクを一括追加（entry は init なし）
    pub fn add_tasks(ctx: Context<AddTasks>, titles: Vec<String>) -> Result<()> {
        let list = &mut ctx.accounts.task_list;
        let entry = &mut ctx.accounts.entry;

        for t in titles.iter() {
            // 空文字でないものだけカウント
            if t.len() > 0 {
                list.count += 1;
                // 最新のタイトルを entry に格納
                entry.title = t.clone();
            }
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitList<'info> {
    #[account(init, payer = user, space = 8 + 4)]
    pub task_list: Account<'info, TaskList>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddTasks<'info> {
    pub task_list: Account<'info, TaskList>,  // ← init なし：既存参照のみ
    pub entry: Account<'info, TaskEntry>,     // ← init なし（本来は初期化すべき）
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct TaskList {
    pub count: u32,
}

#[account]
pub struct TaskEntry {
    pub title: String,
}
