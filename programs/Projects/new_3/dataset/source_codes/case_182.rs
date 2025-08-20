use anchor_lang::prelude::*;
declare_id!("TimeTrack1111111111111111111111111111111");

/// プロジェクト情報
#[account]
pub struct Project {
    pub manager: Pubkey, // プロジェクト管理者
    pub name:    String, // プロジェクト名
}

/// 時間記録エントリ
#[account]
pub struct TimeEntry {
    pub user:     Pubkey, // 記録者
    pub project:  Pubkey, // 本来は Project.key() と一致すべき
    pub details:  String, // 作業内容メモ
}

#[derive(Accounts)]
pub struct CreateProject<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 4 + 64)]
    pub project:      Account<'info, Project>,
    #[account(mut)]
    pub manager:      Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct LogTime<'info> {
    /// Project.manager == manager.key() は検証される
    #[account(mut, has_one = manager)]
    pub project:      Account<'info, Project>,

    /// TimeEntry.project ⇔ project.key() の検証が **ない**
    #[account(init, payer = user, space = 8 + 32 + 32 + 4 + 128)]
    pub time_entry:   Account<'info, TimeEntry>,

    #[account(mut)]
    pub manager:      Signer<'info>, // 作業者としてログに入れているユーザー
    #[account(mut)]
    pub user:         Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[program]
pub mod time_tracking_vuln {
    use super::*;

    /// プロジェクトを新規作成
    pub fn create_project(ctx: Context<CreateProject>, name: String) -> Result<()> {
        let p = &mut ctx.accounts.project;
        p.manager = ctx.accounts.manager.key();
        p.name    = name;
        Ok(())
    }

    /// 作業時間を記録
    pub fn log_time(ctx: Context<LogTime>, details: String) -> Result<()> {
        let e = &mut ctx.accounts.time_entry;
        let p = &ctx.accounts.project;
        // 脆弱性ポイント:
        // e.project = p.key(); と代入するだけで、
        // TimeEntry.project と Project.key() の一致検証がない
        e.user    = ctx.accounts.user.key();
        e.project = p.key();
        e.details = details;
        Ok(())
    }
}
