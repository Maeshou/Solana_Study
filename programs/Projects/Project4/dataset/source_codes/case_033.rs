use anchor_lang::prelude::*;

declare_id!("Ex9000000000000000000000000000000000009");

#[program]
pub mod example9 {
    use super::*;

    // プロジェクトを開始し、作成時刻とアクティブフラグを設定
    pub fn start_project(ctx: Context<StartProj>, id: u64) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let p = &mut ctx.accounts.project;          // ← initあり
        p.id = id;
        p.active = true;
        p.created_at = now;
        p.log_count = 0;
        Ok(())
    }

    // ログを追加し、ログ件数と最終ログを更新
    pub fn add_log(ctx: Context<AddLog>, msg: String) -> Result<()> {
        let pr = &mut ctx.accounts.project;          // ← initなし：既存参照のみ
        if pr.active {
            pr.log_count += 1;
            pr.last_log = msg.clone();
            pr.last_logged_at = Clock::get()?.unix_timestamp;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartProj<'info> {
    #[account(init, payer = mgr, space = 8 + 8*3 + 4)]
    pub project: Account<'info, ProjectData>,
    #[account(mut)] pub mgr: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddLog<'info> {
    pub project: Account<'info, ProjectData>,
    #[account(mut)] pub mgr: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ProjectData {
    pub id: u64,
    pub active: bool,
    pub created_at: i64,
    pub log_count: u32,
    pub last_log: String,
    pub last_logged_at: i64,
}
