use anchor_lang::prelude::*;

// ── アカウントデータはファイル冒頭にタプル構造体で定義 ──
#[account]
#[derive(Default)]
pub struct TaskBoard(pub u8, pub Vec<(u64, Pubkey)>); // (bump, Vec<(task_id, assignee)>)

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzVL");

#[error_code]
pub enum ErrorCode {
    #[msg("Maximum number of tasks reached")]
    MaxTasksReached,
    #[msg("Task not found")]
    TaskNotFound,
}

#[program]
pub mod task_board {
    use super::*;

    const MAX_TASKS: usize = 30;

    /// ボード初期化：内部 Vec は空、bump のみ設定
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let b = *ctx.bumps.get("board").unwrap();
        ctx.accounts.board.0 = b;
        Ok(())
    }

    /// タスク追加：件数制限チェック＋assignee なしで追加
    pub fn add_task(ctx: Context<Modify>, task_id: u64) -> Result<()> {
        let list = &mut ctx.accounts.board.1;
        if list.len() >= MAX_TASKS {
            return err!(ErrorCode::MaxTasksReached);
        }
        list.push((task_id, Pubkey::default()));
        Ok(())
    }

    /// タスク割当：既存タスク探索＋担当者更新
    pub fn assign_task(ctx: Context<Modify>, task_id: u64, assignee: Pubkey) -> Result<()> {
        let list = &mut ctx.accounts.board.1;
        let mut found = false;
        for entry in list.iter_mut() {
            if entry.0 == task_id {
                entry.1 = assignee;
                found = true;
            }
        }
        if found == false {
            return err!(ErrorCode::TaskNotFound);
        }
        Ok(())
    }

    /// タスク削除：該当タスクを一括除去
    pub fn remove_task(ctx: Context<Modify>, task_id: u64) -> Result<()> {
        let list = &mut ctx.accounts.board.1;
        list.retain(|&(id, _)| {
            if id == task_id {
                false
            } else {
                true
            }
        });
        Ok(())
    }

    /// タスク数報告：ログ出力
    pub fn count_tasks(ctx: Context<Modify>) -> Result<()> {
        let cnt = ctx.accounts.board.1.len() as u64;
        msg!("Total tasks: {}", cnt);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_zeroed,
        payer = authority,
        seeds = [b"board", authority.key().as_ref()],
        bump,
        // discriminator(8) + bump(1) + Vec len(4) + max30*(8+32)
        space = 8 + 1 + 4 + 30 * (8 + 32)
    )]
    pub board:     Account<'info, TaskBoard>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Modify<'info> {
    #[account(
        mut,
        seeds = [b"board", authority.key().as_ref()],
        bump = board.0,
    )]
    pub board:     Account<'info, TaskBoard>,
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
