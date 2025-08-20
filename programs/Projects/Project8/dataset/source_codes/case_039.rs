use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_game {
    use super::*;
    // 複数のアイテムを合成して新しいアイテムを生成
    pub fn craft_new_item(ctx: Context<CraftNewItem>, recipe_id: u32, amount: u8) -> Result<()> {
        let inventory = &mut ctx.accounts.player_inventory;
        let recipe = &ctx.accounts.crafting_recipe;

        // レシピIDが一致するか確認
        require!(recipe.recipe_id == recipe_id, GameError::InvalidRecipe);
        
        let mut required_item_count = 0;
        // 必要な素材がインベントリに存在するかループで確認
        for required_item in recipe.required_items.iter() {
            let mut found = false;
            for owned_item in inventory.items.iter() {
                if owned_item.item_id == required_item.item_id {
                    if owned_item.quantity >= required_item.quantity * u64::from(amount) {
                       found = true;
                       required_item_count += 1;
                    }
                }
            }
            require!(found, GameError::InsufficientMaterials);
        }
        
        // すべての素材が見つかった場合のみ処理を続行
        if required_item_count == recipe.required_items.len() {
            // 素材を消費
            for required_item in recipe.required_items.iter() {
                for owned_item in inventory.items.iter_mut() {
                    if owned_item.item_id == required_item.item_id {
                        owned_item.quantity -= required_item.quantity * u64::from(amount);
                    }
                }
            }

            // 新しいアイテムをインベントリに追加
            let mut item_added = false;
            for owned_item in inventory.items.iter_mut() {
                if owned_item.item_id == recipe.result_item_id {
                    owned_item.quantity += u64::from(recipe.result_item_quantity * amount);
                    item_added = true;
                }
            }

            // インベントリに同じアイテムがなかった場合、新しく追加
            if !item_added {
                 inventory.items.push(InventoryItem {
                    item_id: recipe.result_item_id,
                    quantity: u64::from(recipe.result_item_quantity * amount),
                });
            }

            msg!("Successfully crafted item {}!", recipe.result_item_id);
        }

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(recipe_id: u32)]
pub struct CraftNewItem<'info> {
    #[account(mut, seeds = [b"inventory", player.key().as_ref()], bump = player_inventory.bump)]
    pub player_inventory: Account<'info, PlayerInventory>,
    #[account(seeds = [b"recipe", recipe_id.to_le_bytes().as_ref()], bump = crafting_recipe.bump)]
    pub crafting_recipe: Account<'info, CraftingRecipe>,
    #[account(mut)]
    pub player: Signer<'info>,
}

#[account]
pub struct PlayerInventory {
    pub items: Vec<InventoryItem>,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InventoryItem {
    pub item_id: u32,
    pub quantity: u64,
}

#[account]
pub struct CraftingRecipe {
    pub recipe_id: u32,
    pub required_items: Vec<InventoryItem>,
    pub result_item_id: u32,
    pub result_item_quantity: u8,
    pub bump: u8,
}


#[error_code]
pub enum GameError {
    #[msg("The provided recipe is invalid.")]
    InvalidRecipe,
    #[msg("Insufficient materials to craft the item.")]
    InsufficientMaterials,
}