use anchor_lang::prelude::*;

declare_id!("NoPushTodo888888888888888888888888888888888");

#[program]
pub mod todo_app {
    use super::*;

    pub fn init_list(ctx: Context<InitList>) -> Result<()> {
        let l = &mut ctx.accounts.todo_list;
        l.item = "".to_string();
        l.done = false;
        Ok(())
    }

    pub fn update(ctx: Context<UpdateItem>, text: String) -> Result<()> {
        // todo_list に init がない → 別リストの上書き可
        let l = &mut ctx.accounts.todo_list;
        l.item = text;
        // status_account を毎回 init → 再初期化される
        let s = &mut ctx.accounts.status_account;
        s.flag = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitList<'info> {
    #[account(init, payer = user, space = 8 + 256 + 1)]
    pub todo_list: Account<'info, TodoData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateItem<'info> {
    pub todo_list: Account<'info, TodoData>,
    #[account(mut, init, payer = user, space = 8 + 1)]
    pub status_account: Account<'info, StatusData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct TodoData {
    pub item: String,
    pub done: bool,
}

#[account]
pub struct StatusData {
    pub flag: bool,
}
