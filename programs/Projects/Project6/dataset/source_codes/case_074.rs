use anchor_lang::prelude::*;

declare_id!("REPAR444444444444444444444444444444444444444");

#[program]
pub mod repair_station_program {
    use super::*;
    /// 「修理キット」を消費し、武器の耐久度を回復させます。
    pub fn repair_item_durability(ctx: Context<RepairItem>, kit_amount_to_use: u32) -> Result<()> {
        let weapon = &mut ctx.accounts.weapon_to_repair;
        let inventory = &mut ctx.accounts.player_inventory;
        let repair_kit_id = 777;
        let durability_per_kit = 25;

        let total_recovery = durability_per_kit.saturating_mul(kit_amount_to_use);
        let recovered_durability = weapon.current_durability.saturating_add(total_recovery);
        weapon.current_durability = recovered_durability.min(weapon.max_durability);

        let mut kits_to_consume = kit_amount_to_use;
        inventory.items.iter_mut()
            .filter(|item| item.item_id == repair_kit_id)
            .for_each(|kit| {
                let amount_to_use = kit.quantity.min(kits_to_consume);
                kit.quantity -= amount_to_use;
                kits_to_consume -= amount_to_use;
            });
        inventory.items.retain(|item| item.quantity > 0);

        msg!("Weapon durability repaired to {}", weapon.current_durability);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RepairItem<'info> {
    #[account(mut, has_one = owner)]
    pub weapon_to_repair: Account<'info, Weapon>,
    #[account(mut, has_one = owner)]
    pub player_inventory: Account<'info, PlayerInventory>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Weapon {
    pub owner: Pubkey,
    pub current_durability: u32,
    pub max_durability: u32,
}

#[account]
pub struct PlayerInventory {
    pub owner: Pubkey,
    pub items: Vec<InventoryItem>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InventoryItem {
    pub item_id: u32,
    pub quantity: u32,
}