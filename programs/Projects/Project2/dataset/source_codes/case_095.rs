use anchor_lang::prelude::*;

declare_id!("AuthAct44444444444444444444444444444444");

#[program]
pub mod auth_action44 {
    use super::*;

    /// 記録者のみが呼び出せるフラグ設定
    pub fn do_action(ctx: Context<Action>) -> Result<()> {
        let rec = &mut ctx.accounts.record;
        require_keys_eq!(rec.author, ctx.accounts.user.key(), AuthError::NotAllowed);
        rec.active = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Action<'info> {
    #[account(mut, has_one = author)]
    pub record: Account<'info, ActionRecord>,
    pub user: Signer<'info>,
}

#[account]
pub struct ActionRecord {
    pub author: Pubkey,
    pub active: bool,
}

#[error_code]
pub enum AuthError {
    #[msg("権限がありません")]
    NotAllowed,
}
