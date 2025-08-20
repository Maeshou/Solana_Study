use anchor_lang::prelude::*;
declare_id!("StockMgmtHasOne1111111111111111111111111111111");

/// 倉庫情報
#[account]
pub struct Warehouse {
    pub manager: Pubkey, // 倉庫管理者
    pub capacity: u64,   // 空き容量
}

/// 在庫エントリ
#[account]
pub struct StockEntry {
    pub item_id:   u64,    // 商品ID
    pub warehouse: Pubkey, // 本来は Warehouse.key() と一致すべき
    pub quantity:  u64,    // 在庫数
}

#[derive(Accounts)]
pub struct InitializeWarehouse<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 8)]
    pub warehouse: Account<'info, Warehouse>,
    #[account(mut)]
    pub manager:   Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddStock<'info> {
    /// Warehouse.manager == manager.key() は検証される
    #[account(mut, has_one = manager)]
    pub warehouse:   Account<'info, Warehouse>,

    /// StockEntry.warehouse == warehouse.key() の検証がない
    #[account(init, payer = manager, space = 8 + 8 + 32 + 8)]
    pub stock_entry: Account<'info, StockEntry>,

    pub manager:     Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RemoveStock<'info> {
    /// Warehouse.manager == manager.key() は検証される
    #[account(mut, has_one = manager)]
    pub warehouse:   Account<'info, Warehouse>,

    /// StockEntry.warehouse と warehouse.key() の一致チェックがない
    #[account(mut)]
    pub stock_entry: Account<'info, StockEntry>,

    pub manager:     Signer<'info>,
}

#[program]
pub mod stock_vuln_hasone {
    use super::*;

    /// 倉庫を初期化
    pub fn initialize_warehouse(ctx: Context<InitializeWarehouse>, capacity: u64) -> Result<()> {
        let wh = &mut ctx.accounts.warehouse;
        wh.manager  = ctx.accounts.manager.key();
        wh.capacity = capacity;
        Ok(())
    }

    /// 在庫を追加
    pub fn add_stock(ctx: Context<AddStock>, item_id: u64, qty: u64) -> Result<()> {
        let wh = &mut ctx.accounts.warehouse;
        let se = &mut ctx.accounts.stock_entry;
        // 脆弱性ポイント：
        // se.warehouse = wh.key(); と設定するだけで、
        // StockEntry.warehouse と Warehouse.key() の一致検証がない
        se.item_id   = item_id;
        se.warehouse = wh.key();
        se.quantity  = qty;
        wh.capacity  = wh.capacity.checked_sub(qty).unwrap();
        Ok(())
    }

    /// 在庫を出庫
    pub fn remove_stock(ctx: Context<RemoveStock>, qty: u64) -> Result<()> {
        let wh = &mut ctx.accounts.warehouse;
        let se = &mut ctx.accounts.stock_entry;
        // 本来は必須：
        // require_keys_eq!(
        //     se.warehouse,
        //     wh.key(),
        //     ErrorCode::WarehouseMismatch
        // );
        se.quantity = se.quantity.checked_sub(qty).unwrap();
        wh.capacity = wh.capacity.checked_add(qty).unwrap();
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("StockEntry が指定の Warehouse と一致しません")]
    WarehouseMismatch,
}
