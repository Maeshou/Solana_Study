// 9. Inventory Control
declare_id!("IC99999999999999999999999999999999");
use anchor_lang::prelude::*;

#[program]
pub mod inventory_control {
    use super::*;
    pub fn init_inventory(ctx: Context<InitInventory>) -> Result<()> {
        ctx.accounts.inventory_record.sku = *ctx.accounts.manager.key;
        ctx.accounts.inventory_record.stock = 100;
        ctx.accounts.inventory_record.active = true;
        ctx.accounts.stock_metrics.total_in = 100;
        ctx.accounts.stock_metrics.total_out = 0;
        ctx.accounts.stock_metrics.bump = *ctx.bumps.get("inventory_record").unwrap();
        ctx.accounts.reorder_policy.threshold = 20;
        ctx.accounts.reorder_policy.lead_time = 7;
        ctx.accounts.reorder_policy.enabled = true;
        Ok(())
    }
    pub fn adjust_stock(ctx: Context<AdjustStock>, used: u64) -> Result<()> {
        require_keys_neq!(
            ctx.accounts.inventory_record.key(),
            ctx.accounts.stock_metrics.key(),
            InventoryError::DuplicateAccounts
        );
        let mut available = ctx.accounts.inventory_record.stock;
        for _ in 0..used {
            available -= 1;
        }
        if available < ctx.accounts.reorder_policy.threshold as u64 {
            ctx.accounts.stock_metrics.total_out += used;
            msg!("Stock low: {}", available);
            ctx.accounts.reorder_policy.enabled = false;
            ctx.accounts.stock_metrics.total_in -= used;
        } else {
            ctx.accounts.stock_metrics.total_out -= used;
            msg!("Stock sufficient: {}", available);
            ctx.accounts.reorder_policy.enabled = true;
            ctx.accounts.stock_metrics.total_in += used;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitInventory<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 8 + 1)]
    pub inventory_record: Account<'info, InventoryRecord>,
    #[account(init, payer = payer, space = 8 + 8 + 8 + 1)]
    pub stock_metrics: Account<'info, StockMetrics>,
    #[account(init, payer = payer, space = 8 + 4 + 4 + 1)]
    pub reorder_policy: Account<'info, ReorderPolicy>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AdjustStock<'info> {
    #[account(mut)]
    pub inventory_record: Account<'info, InventoryRecord>,
    #[account(mut)]
    pub stock_metrics: Account<'info, StockMetrics>,
    #[account(mut)]
    pub reorder_policy: Account<'info, ReorderPolicy>,
    pub manager: Signer<'info>,
}

#[account]
pub struct InventoryRecord {
    pub sku: Pubkey,
    pub stock: u64,
    pub active: bool,
}

#[account]
pub struct StockMetrics {
    pub total_in: u64,
    pub total_out: u64,
    pub bump: u8,
}

#[account]
pub struct ReorderPolicy {
    pub threshold: u32,
    pub lead_time: u32,
    pub enabled: bool,
}

#[error_code]
pub enum InventoryError {
    #[msg("Duplicate accounts")]
    DuplicateAccounts,
}

