use anchor_lang::prelude::*;

declare_id!("OwnChkE6000000000000000000000000000000007");

#[program]
pub mod market_price {
    pub fn update_price(
        ctx: Context<UpdatePrice>,
        price: u64,
    ) -> Result<()> {
        let mp = &mut ctx.accounts.market;
        // 属性レベルで updater を検証
        mp.current = price;
        mp.history.push(price);

        // external_log は unchecked
        ctx.accounts.external_log.data.borrow_mut().extend_from_slice(&price.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdatePrice<'info> {
    #[account(mut, has_one = updater)]
    pub market: Account<'info, MarketData>,
    pub updater: Signer<'info>,
    /// CHECK: 外部データログ、所有者検証なし
    #[account(mut)]
    pub external_log: AccountInfo<'info>,
}

#[account]
pub struct MarketData {
    pub updater: Pubkey,
    pub current: u64,
    pub history: Vec<u64>,
}
