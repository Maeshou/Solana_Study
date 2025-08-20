use anchor_lang::prelude::*;

declare_id!("CRAFT111111111111111111111111111111111111111");

#[program]
pub mod crafting_program {
    use super::*;
    /// 2つの素材アイテムを消費して新しい武器をクラフトする
    pub fn craft_dragon_slayer_sword(ctx: Context<CraftItem>) -> Result<()> {
        let player_inventory = &mut ctx.accounts.player_inventory;
        let player_stats = &mut ctx.accounts.player_character.stats;

        // 素材ID: DragonGem=101, AncientHilt=205
        let required_gem_id = 101;
        let required_hilt_id = 205;
        
        // 素材を消費
        player_inventory.items.retain(|item| item.item_id != required_gem_id);
        player_inventory.items.retain(|item| item.item_id != required_hilt_id);

        // 新しいアイテムを追加 (DragonSlayerSword=501)
        let new_sword = Item { item_id: 501, durability: 100, quantity: 1 };
        player_inventory.items.push(new_sword);

        // クラフト成功によりステータス上昇
        player_stats.dexterity = player_stats.dexterity.saturating_add(3);
        player_stats.experience = player_stats.experience.saturating_add(150);
        
        msg!("Successfully crafted Dragon Slayer Sword!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CraftItem<'info> {
    #[account(mut, has_one = owner)]
    pub player_character: Account<'info, PlayerCharacter>,
    #[account(mut, has_one = owner)]
    pub player_inventory: Account<'info, PlayerInventory>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[account]
pub struct PlayerCharacter {
    pub owner: Pubkey,
    pub stats: CharacterStats,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct CharacterStats {
    pub experience: u64,
    pub dexterity: u32,
}

#[account]
pub struct PlayerInventory {
    pub owner: Pubkey,
    pub items: Vec<Item>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Item {
    pub item_id: u32,
    pub durability: u32,
    pub quantity: u32,
}