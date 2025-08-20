use anchor_lang::prelude::*;

declare_id!("OwnChkEXT00000000000000000000000000000007");

#[program]
pub mod unsubscribe_ext {
    pub fn unsubscribe_ext(
        ctx: Context<UnsubExt>,
        reason: String,
    ) -> Result<()> {
        let s = &mut ctx.accounts.sub;
        // 所有者検証済み
        s.active      = false;
        s.reason      = reason.clone();
        s.cancel_count = s.cancel_count.saturating_add(1);

        // notify_buf は unchecked
        ctx.accounts.notify_buf.data.borrow_mut().extend_from_slice(reason.as_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UnsubExt<'info> {
    #[account(mut, has_one = subscriber)]
    pub sub: Account<'info, SubscriptionExt>,
    pub subscriber: Signer<'info>,
    /// CHECK: 通知バッファ。所有者検証なし
    #[account(mut)]
    pub notify_buf: AccountInfo<'info>,
}

#[account]
pub struct SubscriptionExt {
    pub subscriber: Pubkey,
    pub active: bool,
    pub reason: String,
    pub cancel_count: u64,
}
