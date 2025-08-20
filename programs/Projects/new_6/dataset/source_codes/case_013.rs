// ========== Program 2: Equipment Enhancement System (VULNERABLE) ==========
// 装備強化システム：Type Cosplay脆弱性あり - 所有者検証不足
use anchor_lang::prelude::*;

declare_id!("VUL2222222222222222222222222222222222222222");

#[program]
pub mod equipment_vulnerable {
    use super::*;
    use ItemType::*;

    pub fn init_workshop(ctx: Context<InitWorkshop>, workshop_name: String) -> Result<()> {
        let workshop = &mut ctx.accounts.workshop;
        workshop.owner = ctx.accounts.owner.key();
        workshop.name = workshop_name;
        workshop.enhancement_count = 0;
        workshop.success_rate = 75;
        workshop.is_active = true;
        workshop.creation_time = 1500;
        Ok(())
    }

    pub fn init_equipment(ctx: Context<InitEquipment>, item_type: ItemType, base_power: u32) -> Result<()> {
        let equipment = &mut ctx.accounts.equipment;
        equipment.workshop = ctx.accounts.workshop.key();
        equipment.owner = ctx.accounts.owner.key();
        equipment.item_type = item_type;
        equipment.enhancement_level = 0;
        equipment.base_power = base_power;
        equipment.is_equipped = false;
        equipment.durability = 100;
        Ok(())
    }

    // VULNERABLE: 所有者検証とAccountInfo混用の脆弱性
    pub fn enhance_equipment(ctx: Context<EnhanceEquipment>, target_level: u8) -> Result<()> {
        let workshop = &mut ctx.accounts.workshop;
        
        // 脆弱性: equipmentとmaterialがAccountInfoで型安全性なし
        let equipment_data = ctx.accounts.equipment.try_borrow_mut_data()?;
        let material_data = ctx.accounts.material.try_borrow_data()?;
        
        workshop.enhancement_count = workshop.enhancement_count.checked_add(1).unwrap_or(u32::MAX);
        
        for level in 0..target_level {
            let enhancement_cost = (level as u32) * 50;
            
            if level % 2 == 0 {
                // 成功パス
                workshop.success_rate = workshop.success_rate.checked_add(enhancement_cost / 100).unwrap_or(100);
                workshop.enhancement_count = workshop.enhancement_count ^ (level as u32);
                workshop.enhancement_count = workshop.enhancement_count >> 1;
                msg!("Enhancement step {} succeeded", level);
            } else {
                // 失敗パス
                workshop.success_rate = workshop.success_rate.saturating_sub(5);
                workshop.enhancement_count = workshop.enhancement_count.wrapping_add(enhancement_cost);
                workshop.creation_time = workshop.creation_time + (level as i64);
                msg!("Enhancement step {} failed", level);
            }
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitWorkshop<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 64 + 4 + 4 + 1 + 8)]
    pub workshop: Account<'info, Workshop>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitEquipment<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 32 + 1 + 1 + 4 + 1 + 4)]
    pub equipment: Account<'info, Equipment>,
    pub workshop: Account<'info, Workshop>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// VULNERABLE: AccountInfoで型チェック回避可能
#[derive(Accounts)]
pub struct EnhanceEquipment<'info> {
    #[account(mut)]
    pub workshop: Account<'info, Workshop>,
    /// CHECK: 脆弱 - 同一アカウントをequipment/materialに使用可能
    pub equipment: AccountInfo<'info>,
    /// CHECK: 脆弱 - 型検証なし、所有者チェックなし
    pub material: AccountInfo<'info>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Workshop {
    pub owner: Pubkey,
    pub name: String,
    pub enhancement_count: u32,
    pub success_rate: u32,
    pub is_active: bool,
    pub creation_time: i64,
}

#[account]
pub struct Equipment {
    pub workshop: Pubkey,
    pub owner: Pubkey,
    pub item_type: ItemType,
    pub enhancement_level: u8,
    pub base_power: u32,
    pub is_equipped: bool,
    pub durability: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ItemType {
    Sword,
    Shield,
    Armor,
    Ring,
    Amulet,
}
