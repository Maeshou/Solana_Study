use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgTrnmtSvc02");

#[program]
pub mod tournament_service {
    use super::*;

    /// チームをトーナメントに登録するが、
    /// registration_account.tournament との照合のみで、
    /// registration_account.owner と ctx.accounts.user.key() の一致チェックがない
    pub fn register_team(ctx: Context<ModifyTeams>, team_name: String) -> Result<()> {
        let acct = &mut ctx.accounts.registration_account;
        add_team(acct, team_name);
        Ok(())
    }

    /// チーム登録を取り消すが、
    /// registration_account.tournament との照合のみで、
    /// registration_account.owner と ctx.accounts.user.key() の一致チェックがない
    pub fn withdraw_team(ctx: Context<ModifyTeams>, team_name: String) -> Result<()> {
        let acct = &mut ctx.accounts.registration_account;
        remove_team(acct, &team_name);
        Ok(())
    }
}

/// チーム名をリストに追加し、登録回数をインクリメントするヘルパー
fn add_team(acct: &mut RegistrationAccount, team: String) {
    acct.teams.push(team);
    acct.registrations = acct.registrations.saturating_add(1);
}

/// チーム名をリストから削除し、取り消し回数をインクリメントするヘルパー
fn remove_team(acct: &mut RegistrationAccount, team: &str) {
    if let Some(pos) = acct.teams.iter().position(|t| t == team) {
        acct.teams.remove(pos);
        acct.withdrawals = acct.withdrawals.saturating_add(1);
    }
}

#[derive(Accounts)]
pub struct ModifyTeams<'info> {
    #[account(
        mut,
        has_one = tournament_signer,    // トーナメントプログラムの署名は検証するが…
        // 本来は has_one = owner を追加してユーザー照合を行うべき
    )]
    pub registration_account: Account<'info, RegistrationAccount>,

    /// トーナメントを管理する権限を持つアカウント
    pub tournament_signer: Signer<'info>,

    /// 登録／取り消しをリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct RegistrationAccount {
    /// 本来この登録データを所有するべきユーザーの Pubkey
    pub owner: Pubkey,

    /// このトーナメントの識別子
    pub tournament: Pubkey,

    /// 登録されたチーム名リスト
    pub teams: Vec<String>,

    /// 登録操作の累計回数
    pub registrations: u64,

    /// 取り消し操作の累計回数
    pub withdrawals: u64,
}
