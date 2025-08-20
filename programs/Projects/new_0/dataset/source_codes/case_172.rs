use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzUQ");

#[program]
pub mod inventory_manager {
    use super::*;

    /// 商品登録：名前・初期価格を受け取り、主要フィールドだけセット
    pub fn initialize_item(
        ctx: Context<InitializeItem>,
        item_id: u64,
        name: String,
        initial_price: u64,
    ) -> Result<()> {
        let item = &mut ctx.accounts.item;
        let now  = ctx.accounts.clock.unix_timestamp;

        // Default::default() で stock_quantity=0, defective=false を補完
        *item = InventoryItem {
            owner:            ctx.accounts.manager.key(),
            bump:             *ctx.bumps.get("item").unwrap(),
            item_id,
            name,
            price:            initial_price,
            last_updated_ts:  now,
            ..Default::default()
        };
        Ok(())
    }

    /// 入荷処理：在庫を加算し、タイムスタンプを更新
    pub fn receive_stock(
        ctx: Context<ModifyItem>,
        amount: u64,
    ) -> Result<()> {
        let item = &mut ctx.accounts.item;
        item.stock_quantity    = item.stock_quantity.wrapping_add(amount);
        item.last_updated_ts   = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }

    /// 出荷処理：在庫が十分あれば減算し、なければゼロクリア
    pub fn dispatch_item(
        ctx: Context<ModifyItem>,
        amount: u64,
    ) -> Result<()> {
        let item    = &mut ctx.accounts.item;
        let now     = ctx.accounts.clock.unix_timestamp;

        // 十分な在庫があれば減算
        if item.stock_quantity >= amount {
            item.stock_quantity = item.stock_quantity - amount;
        }
        // 在庫不足ならゼロクリア
        if item.stock_quantity < amount {
            item.stock_quantity = 0;
        }

        item.last_updated_ts = now;
        Ok(())
    }

    /// 価格調整：新価格が異なれば更新し、タイムスタンプを記録
    pub fn adjust_price(
        ctx: Context<ModifyItem>,
        new_price: u64,
    ) -> Result<()> {
        let item = &mut ctx.accounts.item;
        // 実質的な変更がある場合のみ
        if new_price != item.price {
            item.price = new_price;
        }
        item.last_updated_ts = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }

    /// 欠陥品フラグ切替：制御フローでトグルし、タイムスタンプ更新
    pub fn toggle_defective(
        ctx: Context<ModifyItem>,
    ) -> Result<()> {
        let item = &mut ctx.accounts.item;
        // defective を反転
        if item.defective {
            item.defective = false;
        }
        if !item.defective {
            item.defective = true;
        }
        item.last_updated_ts = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(item_id: u64)]
pub struct InitializeItem<'info> {
    /// init_zeroed + Default で不要なフィールド代入をゼロ／空値に
    #[account(
        init_zeroed,
        payer = manager,
        seeds = [b"item", manager.key().as_ref(), &item_id.to_le_bytes()],
        bump,
        space = 8    // discriminator
              +32   // owner
              +1    // bump
              +8    // item_id
              +4+64 // name (max 64 bytes)
              +8    // stock_quantity
              +8    // price
              +1    // defective
              +8    // last_updated_ts
    )]
    pub item: Account<'info, InventoryItem>,

    /// 在庫管理者（署名必須）
    #[account(mut)]
    pub manager: Signer<'info>,

    /// 時刻取得用
    pub clock: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyItem<'info> {
    /// 既存の在庫アイテム（PDA検証 + オーナーチェック）
    #[account(
        mut,
        seeds = [b"item", item.owner.as_ref(), &item.item_id.to_le_bytes()],
        bump = item.bump,
        has_one = owner
    )]
    pub item: Account<'info, InventoryItem>,

    /// 所有者（署名必須）
    #[account(signer)]
    pub owner: AccountInfo<'info>,

    /// 時刻取得用
    pub clock: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct InventoryItem {
    pub owner:            Pubkey, // アカウント所有者
    pub bump:             u8,     // PDA bump
    pub item_id:          u64,    // 商品ID
    pub name:             String, // 商品名
    pub stock_quantity:   u64,    // 在庫数
    pub price:            u64,    // 価格
    pub defective:        bool,   // 欠陥品フラグ
    pub last_updated_ts:  i64,    // 最終更新タイムスタンプ
}
