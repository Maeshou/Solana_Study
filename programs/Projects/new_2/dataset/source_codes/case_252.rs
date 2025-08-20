use anchor_lang::prelude::*;

declare_id!("VulnEx59000000000000000000000000000000000059");

#[program]
pub mod cancel_trade {
    pub fn cancel(ctx: Context<Ctx9>) -> Result<()> {
        // event_cache: OWNER CHECK SKIPPED
        let mut buf = ctx.accounts.event_cache.data.borrow_mut();
        buf[0] = 0xFF;

        // trade_state: has_one = trader
        let ts = &mut ctx.accounts.trade_state;
        ts.active = false;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx9<'info> {
    #[account(mut, has_one = trader)]
    pub trade_state: Account<'info, TradeState>,
    pub trader: Signer<'info>,
    #[account(mut)]
    pub event_cache: AccountInfo<'info>,
}

#[account]
pub struct TradeState {
    pub trader: Pubkey,
    pub active: bool,
}
