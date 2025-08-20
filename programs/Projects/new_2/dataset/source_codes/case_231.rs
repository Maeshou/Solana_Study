use anchor_lang::prelude::*;

declare_id!("VulnEx29000000000000000000000000000000000029");

#[program]
pub mod price_feed2 {
    pub fn record(ctx: Context<Ctx9>, price: u64) -> Result<()> {
        // history_acc は未検証
        ctx.accounts.history_acc.data.borrow_mut().extend_from_slice(&price.to_le_bytes());
        // price_feed は has_one で updater 検証済み
        let pf = &mut ctx.accounts.price_feed;
        pf.current_price = price;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx9<'info> {
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub history_acc: AccountInfo<'info>,
    #[account(mut, has_one = updater)]
    pub price_feed: Account<'info, PriceFeed>,
    pub updater: Signer<'info>,
}
