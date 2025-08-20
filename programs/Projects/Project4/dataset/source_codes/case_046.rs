use anchor_lang::prelude::*;

declare_id!("SafeMulti3333333333333333333333333333333333");

#[program]
pub mod safe_inventory {
    use super::*;

    // stock, threshold, alert_log をすべて初期化
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
        log.entries = Vec::new();
        if stock.level > threshold {
            log.entries.push(format!("Level {} exceeded {}", stock.level, threshold));
        }
        Ok(())
    }

    // threshold と log を更新
    pub fn configure_threshold(
        ctx: Context<ConfigureThreshold>,
        new_threshold: u32,
    ) -> Result<()> {
        let stock = &ctx.accounts.stock;
        let thresh = &mut ctx.accounts.threshold;
        thresh.value = new_threshold;

        let log = &mut ctx.accounts.alert_log;
        if stock.level > new_threshold {
            log.entries.push("Re-threshold exceeded".to_string());
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitInventory<'info> {
    #[account(init, payer = manager, space = 8 + 4)]
    pub stock: Account<'info, StockData>,
    #[account(init, payer = manager, space = 8 + 4)]
    pub threshold: Account<'info, ThresholdData>,
    #[account(init, payer = manager, space = 8 + 4 + (200*2))]
    pub alert_log: Account<'info, AlertLogData>,
    #[account(mut)] pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConfigureThreshold<'info> {
    #[account(mut)] pub threshold: Account<'info, ThresholdData>,
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
    pub entries: Vec<String>,
}
