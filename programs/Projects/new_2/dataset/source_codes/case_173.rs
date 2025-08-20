use anchor_lang::prelude::*;

declare_id!("OwnChkC5000000000000000000000000000000005");

#[program]
pub mod subscription_service {
    pub fn renew(
        ctx: Context<RenewSub>,
        days: u32,
    ) -> Result<()> {
        let s = &mut ctx.accounts.sub;
        // 属性レベルで user を検証
        s.expires_at = s.expires_at.saturating_add(days as u64 * 86400);
        s.renew_count = s.renew_count.saturating_add(1);

        // log_buf は unchecked
        ctx.accounts.log_buf.data.borrow_mut().extend_from_slice(&days.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RenewSub<'info> {
    #[account(mut, has_one = user)]
    pub sub: Account<'info, SubscriptionData>,
    pub user: Signer<'info>,
    /// CHECK: ログバッファ、所有者検証なし
    #[account(mut)]
    pub log_buf: AccountInfo<'info>,
}

#[account]
pub struct SubscriptionData {
    pub user: Pubkey,
    pub expires_at: u64,
    pub renew_count: u64,
}
