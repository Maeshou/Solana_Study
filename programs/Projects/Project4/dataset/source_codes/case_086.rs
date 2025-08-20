use anchor_lang::prelude::*;

declare_id!("SafeEx32PriceAccum11111111111111111111111111");

#[program]
pub mod example32 {
    use super::*;

    pub fn init_accumulator(
        ctx: Context<InitAccumulator>,
        initial_price: u32,
        count: u32,
    ) -> Result<()> {
        let a = &mut ctx.accounts.accum;
        a.sum_price     = initial_price.saturating_mul(count);
        a.entries_count = count;
        a.above_avg     = false;

        // 平均判定
        let avg = if count > 0 { a.sum_price / count } else { 0 };
        if initial_price > avg {
            a.above_avg = true;
        }
        Ok(())
    }

    pub fn add_price(
        ctx: Context<AddPrice>,
        price: u32,
    ) -> Result<()> {
        let a = &mut ctx.accounts.accum;
        a.sum_price = a.sum_price.saturating_add(price);
        a.entries_count = a.entries_count.saturating_add(1);

        let avg = a.sum_price / a.entries_count;
        if price > avg {
            a.above_avg = true;
        } else {
            a.above_avg = false;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitAccumulator<'info> {
    #[account(init, payer = user, space = 8 + 4 + 4 + 1)]
    pub accum: Account<'info, PriceAccumulatorData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddPrice<'info> {
    #[account(mut)] pub accum: Account<'info, PriceAccumulatorData>,
}

#[account]
pub struct PriceAccumulatorData {
    pub sum_price:    u32,
    pub entries_count:u32,
    pub above_avg:    bool,
}
