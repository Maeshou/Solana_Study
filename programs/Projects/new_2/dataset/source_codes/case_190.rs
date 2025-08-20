use anchor_lang::prelude::*;

declare_id!("OwnChkD1000000000000000000000000000000002");

#[program]
pub mod cancel_listing {
    pub fn cancel(
        ctx: Context<Cancel>,
    ) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        // 属性レベルで lister を検証
        listing.active = false;
        listing.cancelled_at = Clock::get()?.unix_timestamp;

        // op_log は unchecked で簡易書き込み
        ctx.accounts.op_log.data.borrow_mut().extend_from_slice(b"cancel;");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Cancel<'info> {
    #[account(mut, has_one = lister)]
    pub listing: Account<'info, Listing>,
    pub lister: Signer<'info>,
    /// CHECK: 操作ログ、所有者検証なし
    #[account(mut)]
    pub op_log: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct Listing {
    pub lister: Pubkey,
    pub active: bool,
    pub cancelled_at: i64,
}
