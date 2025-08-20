use anchor_lang::prelude::*;

declare_id!("SafeEx09TaskTracker11111111111111111111111");

#[program]
pub mod example9 {
    use super::*;

    /// タスク総数と完了数を初期化
    pub fn init_tasks(
        ctx: Context<InitTasks>,
        total: u32,
        done:  u32,
    ) -> Result<()> {
        let t = &mut ctx.accounts.tasks;
        t.total = total;
        t.done  = done.min(total);
        t.pending = t.total - t.done;
        Ok(())
    }

    /// タスク完了を登録
    pub fn complete(
        ctx: Context<Complete>,
        count: u32,
    ) -> Result<()> {
        let t = &mut ctx.accounts.tasks;
        // 一度に完了できる上限をチェック
        let avail = t.total - t.done;
        let to_mark = count.min(avail);
        // ループで一つずつ処理
        let mut i = 0;
        while i < to_mark {
            t.done += 1;
            i += 1;
        }
        t.pending = t.total - t.done;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitTasks<'info> {
    #[account(init, payer = user, space = 8 + 4*3)]
    pub tasks: Account<'info, TaskData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Complete<'info> {
    #[account(mut)] pub tasks: Account<'info, TaskData>,
}

#[account]
pub struct TaskData {
    pub total:   u32,
    pub done:    u32,
    pub pending: u32,
}
