use anchor_lang::prelude::*;
declare_id!("StockMgmt11111111111111111111111111111111111");

/// 倉庫情報
#[account]
pub struct Warehouse {
    pub manager:  Pubkey, // 倉庫管理者
    pub capacity: u64,    // 空き容量
}

/// 在庫エントリ
#[account]
pub struct StockEntry {
    pub item_id:   u64,    // 商品ID
    pub warehouse: Pubkey, // 本来は Warehouse.key() と一致すべき
    pub quantity:  u64,    // 在庫数
}

#[derive(Accounts)]
pub struct AddWarehouse<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 8)]
    pub warehouse:      Account<'info, Warehouse>,
    #[account(mut)]
    pub manager:        Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddStock<'info> {
    /// Warehouse.manager == manager.key() は検証される
    #[account(mut, has_one = manager)]
    pub warehouse:      Account<'info, Warehouse>,

    #[account(init, payer = manager, space = 8 + 8 + 32 + 8)]
    pub stock_entry:    Account<'info, StockEntry>,

    pub manager:        Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DispatchStock<'info> {
    /// Warehouse.manager == manager.key() は検証される
    #[account(mut, has_one = manager)]
    pub warehouse:      Account<'info, Warehouse>,

    /// StockEntry.warehouse == warehouse.key() の検証がないため、
    /// 任意の stock_entry を渡されるとすり抜けられる
    #[account(mut)]
    pub stock_entry:    Account<'info, StockEntry>,

    pub manager:        Signer<'info>,
}

#[program]
pub mod stock_vuln {
    use super::*;

    /// 倉庫を作成
    pub fn add_warehouse(ctx: Context<AddWarehouse>, capacity: u64) -> Result<()> {
        let w = &mut ctx.accounts.warehouse;
        w.manager  = ctx.accounts.manager.key();
        w.capacity = capacity;
        Ok(())
    }

    /// 在庫を追加
    pub fn add_stock(
        ctx: Context<AddStock>,
        item_id: u64,
        quantity: u64
    ) -> Result<()> {
        let w = &mut ctx.accounts.warehouse;
        let s = &mut ctx.accounts.stock_entry;
        // ここで s.warehouse = w.key() と設定するだけで、
        // 本来は一致検証が必要だが省略している
        s.item_id   = item_id;
        s.warehouse = w.key();
        s.quantity  = quantity;
        w.capacity  = w.capacity.checked_sub(quantity).unwrap();
        Ok(())
    }

    /// 在庫を出庫
    pub fn dispatch_stock(
        ctx: Context<DispatchStock>,
        quantity: u64
    ) -> Result<()> {
        let w = &mut ctx.accounts.warehouse;
        let s = &mut ctx.accounts.stock_entry;
        // 本来は以下のチェックが必要：
        // require_keys_eq!(
        //     s.warehouse,
        //     w.key(),
        //     ErrorCode::WarehouseMismatch
        // );
        s.quantity = s.quantity.checked_sub(quantity).unwrap();
        w.capacity = w.capacity.checked_add(quantity).unwrap();
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("StockEntry が指定の Warehouse と一致しません")]
    WarehouseMismatch,
}
