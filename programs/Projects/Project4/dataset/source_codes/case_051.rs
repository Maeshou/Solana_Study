use anchor_lang::prelude::*;

declare_id!("SafeEx02XXXXXXX2222222222222222222222222222");

#[program]
pub mod example2 {
    use super::*;

    pub fn init_inventory(
        ctx: Context<InitInventory>,
        initial: u32,
        threshold: u32,
    ) -> Result<()> {
        let stock = &mut ctx.accounts.stock;
        stock.level = 0;
        for _ in 0..initial {
            stock.level += 1;
        }

        let thresh = &mut ctx.accounts.threshold;
        thresh.value = threshold;

        let log = &mut ctx.accounts.alert_log;
        log.count = if stock.level > threshold {
            stock.level - threshold
        } else {
            0
        };
        log.over = stock.level > threshold;
        Ok(())
    }

    pub fn adjust_stock(
        ctx: Context<AdjustStock>,
        change: i32,
    ) -> Result<()> {
        let stock = &mut ctx.accounts.stock;
        if change >= 0 {
            stock.level = stock.level.saturating_add(change as u32);
        } else {
            stock.level = stock.level.saturating_sub((-change) as u32);
        }

        let log = &mut ctx.accounts.alert_log;
        log.count = if stock.level > ctx.accounts.threshold.value {
            stock.level - ctx.accounts.threshold.value
        } else {
            0
        };
        log.over = stock.level > ctx.accounts.threshold.value;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitInventory<'info> {
    #[account(init, payer = mgr, space = 8 + 4)]
    pub stock: Account<'info, StockData>,
    #[account(init, payer = mgr, space = 8 + 4)]
    pub threshold: Account<'info, ThresholdData>,
    #[account(init, payer = mgr, space = 8 + 4 + 1)]
    pub alert_log: Account<'info, AlertLogData>,
    #[account(mut)] pub mgr: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AdjustStock<'info> {
    #[account(mut)] pub stock: Account<'info, StockData>,
    pub threshold: Account<'info, ThresholdData>,
    #[account(mut)] pub alert_log: Account<'info, AlertLogData>,
}

#[account]
pub struct StockData {
    pub level: u32,
}

#[account]
pub struct ThresholdData {
    pub value: u32,
}

#[account]
pub struct AlertLogData {
    pub count: u32,
    pub over: bool,
}
