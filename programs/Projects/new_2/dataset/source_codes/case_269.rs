use anchor_lang::prelude::*;

#[program]
pub mod inventory_adder {
    use super::*;
    pub fn add_item(
        ctx: Context<AddItem>,
        item_id: u32,
        category: String,
    ) -> Result<()> {
        let inv = &mut ctx.accounts.inventory;

        // アイテムとカテゴリを追加
        inv.items.push(item_id);
        inv.categories.push(category.clone());

        // イベント履歴を残す
        inv.events.push(Event {
            actor: ctx.accounts.manager.key(),
            description: category,
        });

        msg!(
            "Item {} added to inventory by {}",
            item_id,
            ctx.accounts.manager.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddItem<'info> {
    #[account(mut)]
    pub inventory: Account<'info, Inventory>,
    /// CHECK: 管理者想定のサイナー
    pub manager: UncheckedAccount<'info>,
}

#[account]
pub struct Inventory {
    pub items: Vec<u32>,
    pub categories: Vec<String>,
    pub events: Vec<Event>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Event {
    pub actor: Pubkey,
    pub description: String,
}
