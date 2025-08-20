use anchor_lang::prelude::*;

declare_id!("VulnInit3333333333333333333333333333333333");

#[program]
pub mod vuln_inventory {
    use super::*;

    pub fn init_inventory(
        ctx: Context<InitInventory>,
        initial: u32,
        threshold: u32,
    ) -> Result<()> {
        // 在庫をループでセット
        let stock = &mut ctx.accounts.stock;            // ← Init OK
        stock.level = 0;
        for _ in 0..initial {
            stock.level += 1;
        }

        // threshold は init されていない → 任意アドレス差し替え
        let thresh = &mut ctx.accounts.threshold;      // ← Init missing
        thresh.value = threshold;

        let log = &mut ctx.accounts.alert_log;         // ← Init OK
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
    pub threshold: Account<'info, ThresholdData>,   // ← init がない
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
