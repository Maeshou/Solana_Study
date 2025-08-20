use anchor_lang::prelude::*;

declare_id!("MixChk9999999999999999999999999999999999");

#[program]
pub mod mixed_check9 {
    pub fn delete_thread(ctx: Context<Del>) -> Result<()> {
        // thread.creator は検証あり
        require_keys_eq!(ctx.accounts.thread.creator, ctx.accounts.creator.key(), CustomError::Unauthorized);
        ctx.accounts.thread.deleted = true;
        // log_store は検証なし
        let _ = ctx.accounts.log_store.data.borrow();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Del<'info> {
    #[account(mut, has_one = creator)]
    pub thread: Account<'info, ThreadData>,
    pub creator: Signer<'info>,

    /// CHECK: ログストア未検証
    #[account(mut)]
    pub log_store: AccountInfo<'info>,
}

#[account]
pub struct ThreadData {
    pub creator: Pubkey,
    pub deleted: bool,
}
