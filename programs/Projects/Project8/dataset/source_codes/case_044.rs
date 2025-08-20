use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_game {
    use super::*;

    // NFT装備をエンチャントする
    pub fn enchant_equipment(ctx: Context<EnchantEquipment>, scroll_id: u32) -> Result<()> {
        let equipment = &mut ctx.accounts.equipment_nft;
        let inventory = &mut ctx.accounts.player_inventory;
        
        // エンチャントレベルの上限チェック
        require!(equipment.enchant_level < 10, GameError::MaxEnchantLevelReached);
        
        let mut scroll_found = false;
        let mut scroll_index = 0;
        // インベントリからエンチャントスクロールを探す
        for (index, item) in inventory.items.iter().enumerate() {
            if item.item_id == scroll_id {
                if item.quantity > 0 {
                    scroll_found = true;
                    scroll_index = index;
                }
            }
        }
        require!(scroll_found, GameError::ScrollNotFound);
        
        // スクロールを消費
        inventory.items[scroll_index].quantity -= 1;

        // エンチャント成功確率を計算 (レベルが上がるほど低くなる)
        let success_chance = 100 - (equipment.enchant_level * 8); // 例: +0 -> 100%, +1 -> 92%, ...
        
        let clock = Clock::get()?;
        let random_value = (clock.slot % 100) as u8; // 0-99の乱数

        if random_value < success_chance {
            // エンチャント成功
            equipment.enchant_level += 1;
            equipment.bonus_stats += 5; // ステータスボーナス追加
            msg!("Enchantment successful! New enchant level: +{}", equipment.enchant_level);

        } else {
            // エンチャント失敗
            if equipment.enchant_level > 5 {
                // +5以上の場合、ペナルティとしてレベルが1下がる
                equipment.enchant_level -= 1;
                equipment.bonus_stats -= 5;
                msg!("Enchantment failed! Enchant level decreased to +{}.", equipment.enchant_level);
            } else {
                msg!("Enchantment failed! No change in enchant level.");
            }
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct EnchantEquipment<'info> {
    #[account(mut, seeds = [b"equipment", owner.key().as_ref(), equipment_nft.mint.as_ref()], bump = equipment_nft.bump)]
    pub equipment_nft: Account<'info, EquipmentNft>,
    #[account(mut, seeds = [b"inventory", owner.key().as_ref()], bump = player_inventory.bump)]
    pub player_inventory: Account<'info, PlayerInventory>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[account]
pub struct EquipmentNft {
    pub mint: Pubkey,
    pub enchant_level: u8,
    pub bonus_stats: u32,
    pub bump: u8,
}

// PlayerInventoryはパターン2のものを再利用
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


#[error_code]
pub enum GameError {
    #[msg("This equipment has reached the maximum enchant level.")]
    MaxEnchantLevelReached,
    #[msg("Enchant scroll not found in your inventory.")]
    ScrollNotFound,
}