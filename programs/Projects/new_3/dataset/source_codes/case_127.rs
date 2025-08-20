use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgTournRematch03");

#[program]
pub mod tournament_service {
    use super::*;

    /// チームをトーナメントに登録するが、
    /// has_one = tournament_signer のみ検証され、
    /// 実際に登録を行うユーザー（owner/user）照合がないため
    /// 攻撃者が他人のアカウントで任意のチームを登録できる
    pub fn register_team(ctx: Context<ModifyRegistration>, team_name: String) -> Result<()> {
        let acct = &mut ctx.accounts.registration_account;
        acct.last_registered_team = team_name;
        acct.register_count = acct.register_count + 1;
        Ok(())
    }

    /// チーム登録を取り消すが、
    /// has_one = tournament_signer のみ検証され、
    /// 実際に取り消しを行うユーザー照合がないため
    /// 攻撃者が他人のアカウントで不正に取り消し可能
    pub fn withdraw_team(ctx: Context<ModifyRegistration>, team_name: String) -> Result<()> {
        let acct = &mut ctx.accounts.registration_account;
        acct.last_withdrawn_team = team_name;
        acct.withdraw_count = acct.withdraw_count + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ModifyRegistration<'info> {
    #[account(mut, has_one = tournament_signer)]
    /// 本来は has_one = owner (もしくは has_one = user) を追加し、
    /// registration_account.owner と ctx.accounts.user.key() を照合すべき
    pub registration_account: Account<'info, RegistrationAccount>,

    /// トーナメント管理権限を持つアカウント（検証済み）
    pub tournament_signer: Signer<'info>,

    /// 実際の操作を行うユーザー（照合漏れ）
    pub user: Signer<'info>,
}

#[account]
pub struct RegistrationAccount {
    /// 本来この登録データを所有するべきユーザーの Pubkey
    pub owner: Pubkey,

    /// 対象トーナメントの Pubkey
    pub tournament: Pubkey,

    /// 最後に登録したチーム名
    pub last_registered_team: String,

    /// 登録操作の累計回数
    pub register_count: u64,

    /// 最後に取り消したチーム名
    pub last_withdrawn_team: String,

    /// 取り消し操作の累計回数
    pub withdraw_count: u64,
}
