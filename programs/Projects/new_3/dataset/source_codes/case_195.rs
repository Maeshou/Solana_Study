use anchor_lang::prelude::*;
declare_id!("TaskMgmtVuln11111111111111111111111111111");

/// タスクリスト情報
#[account]
pub struct TaskList {
    pub owner:    Pubkey,       // リスト作成者
    pub title:    String,       // リスト名
    pub tasks:    Vec<Pubkey>,  // 含まれるタスク一覧
}

/// タスク割当記録
#[account]
pub struct TaskAssignment {
    pub assignee: Pubkey,       // 割り当て先ユーザー
    pub list:     Pubkey,       // 本来は TaskList.key() と一致すべき
    pub note:     String,       // メモ
}

#[derive(Accounts)]
pub struct CreateList<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 64 + 4 + (32 * 50))]
    pub list:     Account<'info, TaskList>,
    #[account(mut)]
    pub owner:    Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AssignTask<'info> {
    /// TaskList.owner == owner.key() は検証される
    #[account(mut, has_one = owner)]
    pub list:     Account<'info, TaskList>,

    /// TaskAssignment.list ⇔ list.key() の検証がないため、
    /// 偽のレコードで任意のリストにタスクを割り当てられる
    #[account(init, payer = owner, space = 8 + 32 + 32 + 4 + 128)]
    pub record:   Account<'info, TaskAssignment>,

    #[account(mut)]
    pub owner:    Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RemoveAssignment<'info> {
    /// TaskAssignment.assignee == assignee.key() は検証される
    #[account(mut, has_one = assignee)]
    pub record:   Account<'info, TaskAssignment>,

    /// list.key() ⇔ record.list の検証がないため、
    /// 偽物のレコードで別リストからタスクを削除できる
    #[account(mut)]
    pub list:     Account<'info, TaskList>,

    pub assignee: Signer<'info>,
}

#[program]
pub mod task_mgmt_vuln {
    use super::*;

    /// 新しいタスクリストを作成
    pub fn create_list(ctx: Context<CreateList>, title: String) -> Result<()> {
        let lst = &mut ctx.accounts.list;
        lst.owner = ctx.accounts.owner.key();
        lst.title = title;
        Ok(())
    }

    /// タスクを割り当て
    pub fn assign_task(ctx: Context<AssignTask>, assignee: Pubkey, note: String) -> Result<()> {
        let lst = &mut ctx.accounts.list;
        let rec = &mut ctx.accounts.record;

        // 脆弱性ポイント:
        // rec.list = lst.key(); の照合制約がない
        rec.assignee = assignee;
        rec.list     = lst.key();
        rec.note     = note;

        // Vec::push でタスク一覧に追加
        lst.tasks.push(assignee);
        Ok(())
    }

    /// 割当を解除（最後に割り当てられたタスクを除去）
    pub fn remove_assignment(ctx: Context<RemoveAssignment>) -> Result<()> {
        let lst = &mut ctx.accounts.list;

        // 本来必要:
        // require_keys_eq!(ctx.accounts.record.list, lst.key(), ErrorCode::Mismatch);

        // Vec::pop で最後のタスクを削除（分岐・ループなし）
        lst.tasks.pop();
        Ok(())
    }
}
