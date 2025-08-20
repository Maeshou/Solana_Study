use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgItemShp02");

#[program]
pub mod item_shop {
    use super::*;

    /// ユーザーがアイテムを購入するが、
    /// InventoryAccount.owner と ctx.accounts.user.key() の照合チェックがない
    pub fn purchase_item(
        ctx: Context<PurchaseItem>,
        item_id: u64,
        quantity: u64,
    ) -> Result<()> {
        let shop = &ctx.accounts.shop_config;
        let inv = &mut ctx.accounts.inventory;

        // 1. 必要な支払い額を計算 (単価 × 個数)
        let total_cost = shop
            .price_per_item
            .checked_mul(quantity)
            .unwrap();

        // 2. Lamports をユーザーからショッププールへ直接移動
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() -= total_cost;
        **ctx.accounts.shop_treasury.to_account_info().lamports.borrow_mut() += total_cost;

        // 3. インベントリに購入分を登録
        inv.item_ids.push(item_id);
        inv.quantities.push(quantity);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct PurchaseItem<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] などで InventoryAccount.owner と
    /// ctx.accounts.user.key() の一致を検証すべき
    pub inventory: Account<'info, InventoryAccount>,

    /// 購入代金を貯めるショップの財務口座
    #[account(mut)]
    pub shop_treasury: AccountInfo<'info>,

    /// アイテムを購入するユーザー（署名者）
    #[account(mut)]
    pub user: Signer<'info>,

    /// ショップの価格設定アカウント
    pub shop_config: Account<'info, ShopConfig>,
}

#[account]
pub struct InventoryAccount {
    /// このインベントリを所有するユーザーの Pubkey
    pub owner: Pubkey,
    /// 保有アイテムの ID リスト
    pub item_ids: Vec<u64>,
    /// 各アイテムの数量リスト
    pub quantities: Vec<u64>,
}

#[account]
pub struct ShopConfig {
    /// アイテム 1 つあたりの価格 (Lamports)
    pub price_per_item: u64,
}
