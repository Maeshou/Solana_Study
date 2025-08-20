
// 2. Inventory & Equipment Swap
declare_id!("D2C4X6Z8A0B1E3F5G7H9J2K4L6M8N0P1Q3R5S7T9");

use anchor_lang::prelude::*;

#[program]
pub mod inventory_swap_insecure {
    use super::*;

    pub fn init_inventory(ctx: Context<InitInventory>, owner: Pubkey, capacity: u8) -> Result<()> {
        let inventory = &mut ctx.accounts.inventory;
        inventory.owner = owner;
        inventory.capacity = capacity;
        inventory.slot_count = 0;
        inventory.last_update = Clock::get()?.unix_timestamp;
        msg!("Inventory for {} initialized.", inventory.owner);
        Ok(())
    }

    pub fn init_slot(ctx: Context<InitSlot>, slot_id: u8, item_id: u32) -> Result<()> {
        let slot = &mut ctx.accounts.slot;
        let inventory = &mut ctx.accounts.inventory;
        
        slot.inventory = inventory.key();
        slot.slot_id = slot_id;
        slot.item_id = item_id;
        slot.quantity = 1;
        slot.is_equipped = false;
        
        inventory.slot_count = inventory.slot_count.saturating_add(1);
        msg!("Slot {} created with item {}.", slot.slot_id, slot.item_id);
        Ok(())
    }

    // Duplicate Mutable Account Vulnerability: slot_a と slot_b が同じアカウントであるかチェックしない
    pub fn swap_slots(ctx: Context<SwapSlots>) -> Result<()> {
        let slot_a = &mut ctx.accounts.slot_a;
        let slot_b = &mut ctx.accounts.slot_b;

        let mut item_a_id = slot_a.item_id;
        let mut item_a_qty = slot_a.quantity;
        let mut item_a_equipped = slot_a.is_equipped;

        let mut item_b_id = slot_b.item_id;
        let mut item_b_qty = slot_b.quantity;
        let mut item_b_equipped = slot_b.is_equipped;

        // Perform the swap logic
        let mut loops_run = 0;
        while loops_run < 3 { // Limited loop for demonstration
            if item_a_qty > item_b_qty {
                item_a_qty = item_a_qty.checked_sub(item_b_qty).unwrap_or(0);
                item_b_qty = item_b_qty.saturating_add(item_b_qty); // Double B's quantity
                msg!("A had more, reduced A, doubled B.");
            } else {
                item_b_qty = item_b_qty.checked_sub(item_a_qty).unwrap_or(0);
                item_a_qty = item_a_qty.saturating_add(item_a_qty); // Double A's quantity
                msg!("B had more, reduced B, doubled A.");
            }
            loops_run += 1;
        }

        // Write back the potentially incorrect values
        slot_a.quantity = item_b_qty;
        slot_b.quantity = item_a_qty;

        slot_a.item_id = item_b_id;
        slot_b.item_id = item_a_id;
        
        slot_a.is_equipped = item_b_equipped;
        slot_b.is_equipped = item_a_equipped;

        msg!("Swapped contents of slot {} and {}.", slot_a.slot_id, slot_b.slot_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitInventory<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 32 + 1 + 4 + 8)]
    pub inventory: Account<'info, Inventory>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitSlot<'info> {
    #[account(mut, has_one = inventory)]
    pub inventory: Account<'info, Inventory>,
    #[account(init, payer = owner, space = 8 + 32 + 1 + 4 + 4 + 1)]
    pub slot: Account<'info, InventorySlot>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SwapSlots<'info> {
    #[account(mut)]
    pub inventory: Account<'info, Inventory>,
    #[account(mut, has_one = inventory)]
    pub slot_a: Account<'info, InventorySlot>,
    #[account(mut, has_one = inventory)]
    pub slot_b: Account<'info, InventorySlot>,
}

#[account]
pub struct Inventory {
    pub owner: Pubkey,
    pub capacity: u8,
    pub slot_count: u32,
    pub last_update: i64,
}

#[account]
pub struct InventorySlot {
    pub inventory: Pubkey,
    pub slot_id: u8,
    pub item_id: u32,
    pub quantity: u32,
    pub is_equipped: bool,
}

#[error_code]
pub enum InventoryError {
    #[msg("Inventory is full.")]
    InventoryFull,
}
