use anchor_lang::prelude::*;

declare_id!("OwnChkB6000000000000000000000000000000006");

#[program]
pub mod auction_finalize {
    pub fn finalize(ctx: Context<Final>) -> Result<()> {
        let auc = &mut ctx.accounts.auction;
        // has_one で manager チェック済み
        auc.closed = true;
        auc.finalized_at = Clock::get()?.unix_timestamp;

        // bid_log は unchecked
        let mut buf = ctx.accounts.bid_log.data.borrow_mut();
        buf.extend_from_slice(b"finalized;");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Final<'info> {
    #[account(mut, has_one = manager)]
    pub auction: Account<'info, AuctionExt>,
    pub manager: Signer<'info>,
    /// CHECK: 入札ログ、所有者検証なし
    #[account(mut)]
    pub bid_log: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct AuctionExt {
    pub manager: Pubkey,
    pub closed: bool,
    pub finalized_at: i64,
}
