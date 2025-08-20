use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzUH");

#[program]
pub mod order_manager {
    use super::*;

    /// 新規注文作成：注文ID を受け取り、初期状態をまとめて設定
    pub fn create_order(
        ctx: Context<CreateOrder>,
        bump: u8,
        order_id: u64,
    ) -> Result<()> {
        // struct リテラルで一括初期化
        *ctx.accounts.order = Order {
            owner:            ctx.accounts.user.key(),
            bump,
            order_id,
            items_count:      0,
            total_price:      0,
            status:           String::from("created"),
            last_updated_ts:  ctx.accounts.clock.unix_timestamp,
        };
        Ok(())
    }

    /// 商品追加：個数・単価から合計金額を計算し、状態とタイムスタンプも更新
    pub fn add_item(
        ctx: Context<ModifyOrder>,
        quantity: u64,
        price_per_item: u64,
    ) -> Result<()> {
        let o = &mut ctx.accounts.order;
        // 個数累積
        o.items_count   = o.items_count.wrapping_add(quantity);
        // 価格計算
        let added = price_per_item.wrapping_mul(quantity);
        o.total_price   = o.total_price.wrapping_add(added);
        // 状態遷移＆タイムスタンプ更新
        o.status        = String::from("item_added");
        o.last_updated_ts = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }

    /// 注文完了：状態とタイムスタンプを更新
    pub fn complete_order(
        ctx: Context<ModifyOrder>,
    ) -> Result<()> {
        let o = &mut ctx.accounts.order;
        o.status           = String::from("completed");
        o.last_updated_ts  = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }

    /// 注文取消：状態とタイムスタンプを更新
    pub fn cancel_order(
        ctx: Context<ModifyOrder>,
    ) -> Result<()> {
        let o = &mut ctx.accounts.order;
        o.status           = String::from("canceled");
        o.last_updated_ts  = ctx.accounts.clock.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8, order_id: u64)]
pub struct CreateOrder<'info> {
    /// PDA で生成する Order アカウント
    #[account(
        init,
        payer = user,
        // 8(discriminator) + 32(owner) + 1(bump) + 8(order_id)
        // + 8(items_count) + 8(total_price)
        // + 4 + 10(max "completed") // status
        // + 8(last_updated_ts)
        space = 8 + 32 + 1 + 8 + 8 + 8 + 4 + 10 + 8,
        seeds = [b"order", user.key().as_ref(), &order_id.to_le_bytes()],
        bump
    )]
    pub order: Account<'info, Order>,

    /// 注文作成者（署名必須）
    #[account(mut)]
    pub user: Signer<'info>,

    /// システムプログラム
    pub system_program: Program<'info, System>,

    /// 現在のブロック時間を取得
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct ModifyOrder<'info> {
    /// 既存の Order（PDA 検証 + オーナーチェック）
    #[account(
        mut,
        seeds = [b"order", owner.key().as_ref(), &order.order_id.to_le_bytes()],
        bump = order.bump,
        has_one = owner
    )]
    pub order: Account<'info, Order>,

    /// 注文所有者（署名必須）
    #[account(signer)]
    pub owner: AccountInfo<'info>,

    /// システムプログラム
    pub system_program: Program<'info, System>,

    /// タイムスタンプ更新用
    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct Order {
    pub owner:           Pubkey,
    pub bump:            u8,
    pub order_id:        u64,
    pub items_count:     u64,
    pub total_price:     u64,
    pub status:          String,
    pub last_updated_ts: i64,
}
