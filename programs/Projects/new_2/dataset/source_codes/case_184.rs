use anchor_lang::prelude::*;

declare_id!("OwnChkC6000000000000000000000000000000006");

#[program]
pub mod bid_system {
    pub fn place_bid(
        ctx: Context<PlaceBid>,
        amount: u64,
    ) -> Result<()> {
        let auc = &mut ctx.accounts.auction;
        // 属性検証で auc.seller をチェック
        auc.bids.push((ctx.accounts.bidder.key(), amount));
        auc.bid_count = auc.bid_count.saturating_add(1);

        // analytics_buf は unchecked
        let mut buf = ctx.accounts.analytics_buf.data.borrow_mut();
        buf.extend_from_slice(&amount.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PlaceBid<'info> {
    #[account(mut, has_one = seller)]
    pub auction: Account<'info, AuctionData>,
    pub seller: Signer<'info>,
    pub bidder: Signer<'info>,
    /// CHECK: 解析用バッファ、所有者検証なし
    #[account(mut)]
    pub analytics_buf: AccountInfo<'info>,
}

#[account]
pub struct AuctionData {
    pub seller: Pubkey,
    pub bids: Vec<(Pubkey, u64)>,
    pub bid_count: u64,
}
