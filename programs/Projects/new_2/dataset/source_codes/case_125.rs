use anchor_lang::prelude::*;

declare_id!("MixChk6666666666666666666666666666666666");

#[program]
pub mod mixed_check6 {
    pub fn unsubscribe(ctx: Context<Unsub>) -> Result<()> {
        // rec.user は検証あり
        require_keys_eq!(ctx.accounts.rec.user, ctx.accounts.user.key(), CustomError::Forbidden);
        ctx.accounts.rec.active = false;
        // sys_log は検証なし
        let _ = ctx.accounts.sys_log.data.borrow();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Unsub<'info> {
    #[account(mut, has_one = user)]
    pub rec: Account<'info, Subscription>,
    pub user: Signer<'info>,

    /// CHECK: システムログ未検証
    #[account(mut)]
    pub sys_log: AccountInfo<'info>,
}

#[account]
pub struct Subscription {
    pub user: Pubkey,
    pub active: bool,
}
