use anchor_lang::prelude::*;

declare_id!("OwnChkD6000000000000000000000000000000007");

#[program]
pub mod auction_extend {
    pub fn extend_time(
        ctx: Context<Extend>,
        extra_slots: u64,
    ) -> Result<()> {
        let auc = &mut ctx.accounts.auction;
        // 属性レベルで owner を検証
        auc.end_slot = auc.end_slot.saturating_add(extra_slots);
        auc.extend_count = auc.extend_count.saturating_add(1);

        // time_log は unchecked
        ctx.accounts.time_log.data.borrow_mut().extend_from_slice(&extra_slots.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Extend<'info> {
    #[account(mut, has_one = owner)]
    pub auction: Account<'info, AuctionMeta>,
    pub owner: Signer<'info>,
    /// CHECK: タイムログ、所有者検証なし
    #[account(mut)]
    pub time_log: AccountInfo<'info>,
}

#[account]
pub struct AuctionMeta {
    pub owner: Pubkey,
    pub end_slot: u64,
    pub extend_count: u64,
}
