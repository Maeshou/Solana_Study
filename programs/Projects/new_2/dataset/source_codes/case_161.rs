use anchor_lang::prelude::*;

declare_id!("OwnChkB3000000000000000000000000000000003");

#[program]
pub mod listing_manager {
    pub fn remove_listing(ctx: Context<Remove>) -> Result<()> {
        let lst = &mut ctx.accounts.listing;
        // 属性検証で lister をチェック
        lst.active = false;
        lst.removed_at = Clock::get()?.unix_timestamp;

        // op_log は unchecked
        let mut buf = ctx.accounts.op_log.data.borrow_mut();
        buf.extend_from_slice(b"rm;");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Remove<'info> {
    #[account(mut, has_one = lister)]
    pub listing: Account<'info, ListingData>,
    pub lister: Signer<'info>,
    /// CHECK: 操作ログ、所有者検証なし
    #[account(mut)]
    pub op_log: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct ListingData {
    pub lister: Pubkey,
    pub active: bool,
    pub removed_at: i64,
}
