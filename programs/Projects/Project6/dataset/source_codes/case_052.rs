use anchor_lang::prelude::*;

declare_id!("ITMCU777777777777777777777777777777777777777");

#[program]
pub mod item_consumption_program {
    use super::*;
    /// ポーションを複数個同時に使用して体力を回復する
    pub fn consume_multiple_potions(ctx: Context<ConsumePotion>, quantity: u32) -> Result<()> {
        let character_stats = &mut ctx.accounts.player_character.stats;
        let inventory = &mut ctx.accounts.player_inventory;
        
        let health_per_potion = 50;
        let potion_item_id = 1;
        
        // インベントリからポーションを削除
        let potion_entry = inventory.items.iter_mut().find(|item| item.item_id == potion_item_id).unwrap();
        potion_entry.quantity = potion_entry.quantity.saturating_sub(quantity);
        
        for _ in 1..quantity {
             let potential_health = character_stats.current_health.saturating_add(health_per_potion);
             character_stats.current_health = potential_health.min(character_stats.max_health);
        }

        inventory.items.retain(|item| item.quantity > 0);

        msg!("Consumed {} potions.", quantity);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ConsumePotion<'info> {
    #[account(mut, has_one = owner)]
    pub player_character: Account<'info, PlayerCharacter>,
    #[account(mut, has_one = owner, constraint = player_inventory.items.iter().any(|item| item.item_id == 1 && item.quantity >= 1))]
    pub player_inventory: Account<'info, PlayerInventory>,
    pub owner: Signer<'info>,
}

#[account]
pub struct PlayerCharacter {
    pub owner: Pubkey,
    pub stats: CharacterStats,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct CharacterStats {
    pub current_health: u64,
    pub max_health: u64,
}

#[account]
pub struct PlayerInventory {
    pub owner: Pubkey,
    pub items: Vec<Item>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Item {
    pub item_id: u32,
    pub quantity: u32,
}