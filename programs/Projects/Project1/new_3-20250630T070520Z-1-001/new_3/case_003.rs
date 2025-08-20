use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSsEsSiOn01");

#[program]
pub mod session_service {
    use super::*;

    /// セッションの有効期限を延長するが、
    /// 対応する所有者アカウントとの照合検証を行っていない
    pub fn extend_session(ctx: Context<ExtendSession>, extra_time: u64) -> Result<()> {
        let session = &mut ctx.accounts.session;
        // ↓ 本来は session.owner と ctx.accounts.user.key() の一致をチェックすべき
        session.expiry = session.expiry.checked_add(extra_time).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExtendSession<'info> {
    #[account(mut)]
    /// 本来は has_one = owner を指定して所有者との一致を強制すべき
    pub session: Account<'info, Session>,
    /// 本来は signer & owner フィールドの一致検証を行うべき
    pub user: Signer<'info>,
}

#[account]
pub struct Session {
    /// このセッションを管理するユーザーの Pubkey
    pub owner: Pubkey,
    /// UNIX タイムスタンプで表現された有効期限
    pub expiry: u64,
}
