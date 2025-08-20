// 2. Equipment Enhancement & Rarity
declare_id!("D2C4X6Z8A0B1E3F5G7H9J2K4L6M8N0P1Q3R5S7T9");

use anchor_lang::prelude::*;

#[program]
pub mod equip_enhancer {
    use super::*;

    pub fn init_item(ctx: Context<InitItem>, item_id: u64, base_power: u16) -> Result<()> {
        let item = &mut ctx.accounts.item;
        item.owner = ctx.accounts.owner.key();
        item.item_id = item_id;
        item.base_power = base_power;
        item.level = 0;
        item.upgrade_cost = 100;
        item.rarity_level = Rarity::Common;
        item.is_locked = false;
        msg!("Item {} initialized with base power {}", item.item_id, item.base_power);
        Ok(())
    }

    pub fn enhance_item(ctx: Context<EnhanceItem>) -> Result<()> {
        let item = &mut ctx.accounts.item;
        let material = &mut ctx.accounts.material;

        if item.is_locked {
            return Err(ErrorCode::ItemLocked.into());
        }

        let mut success_chance_sum = 0.0;
        let mut total_added_power = 0;
        let mut loops_run = 0;
        let max_loops = 5;

        while loops_run < max_loops {
            if material.count == 0 {
                break;
            }

            let bonus_factor = if item.level > 10 { 1.2 } else { 1.0 };
            let success_roll = 1.0 - (loops_run as f64 * 0.1); // Diminishing returns on success
            
            if success_roll > 0.5 {
                let added_power = (material.power as f64 * bonus_factor) as u16;
                item.base_power = item.base_power.checked_add(added_power).unwrap_or(u16::MAX);
                total_added_power += added_power;
                success_chance_sum += success_roll;
                material.count -= 1;
            } else {
                item.upgrade_cost = item.upgrade_cost.saturating_add(10);
                item.level = item.level.saturating_sub(1);
                material.count = material.count.saturating_sub(1);
                msg!("Enhancement failed, item level decreased and cost increased.");
                break; // Stop on first failure
            }

            loops_run += 1;
        }

        // Conditional rarity upgrade based on total power
        if item.base_power > 2000 {
            item.rarity_level = Rarity::Legendary;
            msg!("Item upgraded to Legendary rarity!");
        } else {
            item.rarity_level = Rarity::Rare;
            msg!("Item upgraded to Rare rarity.");
        }
        
        msg!("Enhanced item {} with {} total added power. New power: {}", item.item_id, total_added_power, item.base_power);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitItem<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 2 + 2 + 1 + 1 + 1)]
    pub item: Account<'info, Equipment>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EnhanceItem<'info> {
    #[account(mut)]
    pub item: Account<'info, Equipment>,
    #[account(mut, has_one = item_parent)]
    pub material: Account<'info, EnhancementMaterial>,
    /// CHECK: Parent is just a pubkey
    pub item_parent: UncheckedAccount<'info>,
}

#[account]
pub struct Equipment {
    pub owner: Pubkey,
    pub item_id: u64,
    pub base_power: u16,
    pub level: u16,
    pub upgrade_cost: u16,
    pub rarity_level: Rarity,
    pub is_locked: bool,
}

#[account]
pub struct EnhancementMaterial {
    pub item_parent: Pubkey,
    pub material_id: u32,
    pub power: u16,
    pub count: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Rarity {
    Common,
    Rare,
    Legendary,
}

#[error_code]
pub enum EnhancementError {
    #[msg("The item is currently locked and cannot be enhanced.")]
    ItemLocked,
}
