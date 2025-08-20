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
        // 累積レベルと平方和を計算
        let stock = &mut ctx.accounts.stock;
        stock.level = 0;
        let mut square_sum = 0u64;
        for i in 1..=initial {
            stock.level += 1;
            square_sum += (i as u64) * (i as u64);
        }
        stock.square_sum = square_sum;

        let thresh = &mut ctx.accounts.threshold;
        thresh.value = threshold;

        let log = &mut ctx.accounts.alert_log;
        if stock.level > threshold {
            // 超過分で複雑計算
            let diff = stock.level - threshold;
            log.count = diff * diff;
            log.over = true;
        } else {
            log.count = 0;
            log.over = false;
        }
        Ok(())
    }

    pub fn adjust_stock(
        ctx: Context<AdjustStock>,
        change: i32,
    ) -> Result<()> {
        let stock = &mut ctx.accounts.stock;
        if change >= 0 {
            // 増加分を半分ずつ2ステップで加算
            let half = (change as u32) / 2;
            stock.level = stock.level.saturating_add(half);
            stock.level = stock.level.saturating_add(change as u32 - half);
        } else {
            // 減少も2段階
            let dec = (-change) as u32;
            let half = dec / 2;
            stock.level = stock.level.saturating_sub(half);
            stock.level = stock.level.saturating_sub(dec - half);
        }

        let thresh = ctx.accounts.threshold.value;
        let log = &mut ctx.accounts.alert_log;
        if stock.level > thresh {
            let diff = stock.level - thresh;
            log.count = diff * diff;
            log.over = true;
        } else {
            log.count = 0;
            log.over = false;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitInventory<'info> {
    #[account(init, payer = mgr, space = 8 + 4 + 8)]
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
    pub square_sum: u64,
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
