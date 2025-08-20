use anchor_lang::prelude::*;

declare_id!("MkFee0555555555555555555555555555555555");

#[program]
pub mod market_fee {
    use super::*;

    pub fn record_trade(ctx: Context<RecordTrade>, sale: u64) -> Result<()> {
        let m = &mut ctx.accounts.market;
        let fee = sale / 100 * m.fee_bps as u64;
        m.accumulated = m.accumulated.saturating_add(fee);
        m.trade_count = m.trade_count.saturating_add(1);
        Ok(())
    }

    pub fn adjust_fee(ctx: Context<AdjustFee>, new_bps: u16) -> Result<()> {
        ctx.accounts.market.fee_bps = new_bps;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RecordTrade<'info> {
    #[account(mut)]
    pub market: Account<'info, MarketFeeData>,
}

#[derive(Accounts)]
pub struct AdjustFee<'info> {
    #[account(mut)]
    pub market: Account<'info, MarketFeeData>,
    pub admin: Signer<'info>,
}

#[account]
pub struct MarketFeeData {
    pub fee_bps: u16,
    pub accumulated: u64,
    pub trade_count: u64,
}
