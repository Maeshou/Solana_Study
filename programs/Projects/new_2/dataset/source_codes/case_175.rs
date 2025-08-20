use anchor_lang::prelude::*;

declare_id!("OwnChkC7000000000000000000000000000000007");

#[program]
pub mod price_feed {
    pub fn update_price(
        ctx: Context<UpdatePrice>,
        price: u64,
    ) -> Result<()> {
        let pf = &mut ctx.accounts.feed;
        // 属性レベルで maintainer を検証
        pf.current_price = price;
        pf.update_count = pf.update_count.saturating_add(1);

        // price_history は unchecked
        let mut hist = ctx.accounts.price_history.data.borrow_mut();
        hist.extend_from_slice(&price.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdatePrice<'info> {
    #[account(mut, has_one = maintainer)]
    pub feed: Account<'info, PriceFeedData>,
    pub maintainer: Signer<'info>,
    /// CHECK: 価格履歴、所有者検証なし
    #[account(mut)]
    pub price_history: AccountInfo<'info>,
}

#[account]
pub struct PriceFeedData {
    pub maintainer: Pubkey,
    pub current_price: u64,
    pub update_count: u64,
}
