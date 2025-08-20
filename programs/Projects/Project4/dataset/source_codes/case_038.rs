use anchor_lang::prelude::*;

declare_id!("InitAll3333333333333333333333333333333333");

#[program]
pub mod multi_init3 {
    use super::*;

    // 在庫をループで積み増し、閾値超過か分岐、アラート発生数を算出
    pub fn init_inventory(
        ctx: Context<InitInventory>,
        initial: u32,
        threshold: u32,
    ) -> Result<()> {
        // stock.level をループで初期化
        let stock = &mut ctx.accounts.stock;
        stock.level = 0;
        let mut i = 0;
        while i < initial {
            stock.level += 1;
            i += 1;
        }

        // threshold
        let thresh = &mut ctx.accounts.threshold;
        thresh.value = threshold;

        // alert_log を分岐で判定
        let log = &mut ctx.accounts.alert_log;
        if stock.level > threshold {
            log.triggered = true;
            log.count = stock.level - threshold;
        } else {
            log.triggered = false;
            log.count = 0;
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
    #[account(init, payer = manager, space = 8 + 1 + 4)]
    pub alert_log: Account<'info, AlertLogData>,
    #[account(mut)] pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
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
    pub triggered: bool,
    pub count: u32,
}
