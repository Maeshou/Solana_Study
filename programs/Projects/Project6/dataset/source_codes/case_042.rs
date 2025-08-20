// ========================================
// 2. 装備強化システム - Equipment Enhancement System
// ========================================

use anchor_lang::prelude::*;

declare_id!("2vTgtyTKjdvH8Cq6YxnHuFcY9Vk8rBn4NmZ7Lp9QsEr3");

#[program]
pub mod equipment_enhancement {
    use super::*;
    use EquipmentType::*;

    pub fn init_forge(ctx: Context<InitForge>) -> Result<()> {
        let forge = &mut ctx.accounts.forge;
        forge.master = ctx.accounts.master.key();
        forge.total_enhancements = 0;
        forge.success_rate = 75;
        forge.operational = true;
        forge.last_maintenance = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn init_equipment(ctx: Context<InitEquipment>, eq_type: EquipmentType, base_power: u32) -> Result<()> {
        let equipment = &mut ctx.accounts.equipment;
        equipment.forge = ctx.accounts.forge.key();
        equipment.owner = ctx.accounts.owner.key();
        equipment.eq_type = eq_type;
        equipment.enhancement_level = 0;
        equipment.base_power = base_power;
        equipment.durability = 100;
        equipment.enchanted = false;
        Ok(())
    }

    pub fn enhance_equipment_batch(ctx: Context<EnhanceEquipment>) -> Result<()> {
        let primary = &mut ctx.accounts.primary_equipment;
        let catalyst = &mut ctx.accounts.catalyst_equipment;
        let forge = &mut ctx.accounts.forge;

        // 連続強化処理ループ
        while primary.enhancement_level < 10 && catalyst.durability > 0 {
            if primary.base_power > catalyst.base_power {
                // プライマリ装備の強化
                let current_level = primary.enhancement_level;
                primary.enhancement_level = current_level.checked_add(1).unwrap_or(255);
                
                // 簡易平方根による効果計算
                let power_bonus = sqrt_newton(primary.base_power as u64) as u32;
                primary.base_power = primary.base_power.checked_add(power_bonus).unwrap_or(u32::MAX);
                
                primary.durability = if primary.durability > 5 { primary.durability - 5 } else { 0 };
                forge.total_enhancements = forge.total_enhancements.checked_add(1).unwrap_or(u64::MAX);
                msg!("Primary enhanced to level: {}", primary.enhancement_level);
            } else {
                // 触媒装備のエネルギー吸収
                let absorption = (catalyst.base_power >> 2) & 0xFF;
                catalyst.base_power = if catalyst.base_power > absorption { catalyst.base_power - absorption } else { 0 };
                catalyst.durability = if catalyst.durability > 10 { catalyst.durability - 10 } else { 0 };
                
                primary.enchanted = !primary.enchanted;
                forge.success_rate = ((forge.success_rate as u64 * 95) / 100).min(100) as u8;
                msg!("Catalyst power absorbed: {}", absorption);
            }
            
            if catalyst.durability == 0 {
                break;
            }
        }

        forge.last_maintenance = Clock::get()?.unix_timestamp;
        Ok(())
    }

    // 簡易ニュートン法による整数平方根
    fn sqrt_newton(n: u64) -> u64 {
        if n == 0 { return 0; }
        let mut x = n;
        let mut y = (x + 1) / 2;
        while y < x {
            x = y;
            y = (x + n / x) / 2;
        }
        x
    }
}

#[derive(Accounts)]
pub struct InitForge<'info> {
    #[account(init, payer = master, space = 8 + 32 + 8 + 1 + 1 + 8)]
    pub forge: Account<'info, Forge>,
    #[account(mut)]
    pub master: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitEquipment<'info> {
    #[account(mut)]
    pub forge: Account<'info, Forge>,
    #[account(init, payer = owner, space = 8 + 32 + 32 + 1 + 1 + 4 + 1 + 1)]
    pub equipment: Account<'info, Equipment>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EnhanceEquipment<'info> {
    #[account(mut, has_one = master)]
    pub forge: Account<'info, Forge>,
    
    #[account(
        mut,
        has_one = forge,
        constraint = primary_equipment.eq_type != catalyst_equipment.eq_type @ EnhanceError::CosplayBlocked,
        owner = crate::ID
    )]
    pub primary_equipment: Account<'info, Equipment>,
    
    #[account(
        mut,
        has_one = forge,
        owner = crate::ID
    )]
    pub catalyst_equipment: Account<'info, Equipment>,
    
    pub master: Signer<'info>,
}

#[account]
pub struct Forge {
    pub master: Pubkey,
    pub total_enhancements: u64,
    pub success_rate: u8,
    pub operational: bool,
    pub last_maintenance: i64,
}

#[account]
pub struct Equipment {
    pub forge: Pubkey,
    pub owner: Pubkey,
    pub eq_type: EquipmentType,
    pub enhancement_level: u8,
    pub base_power: u32,
    pub durability: u8,
    pub enchanted: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Debug)]
pub enum EquipmentType {
    Sword,
    Shield,
    Armor,
    Ring,
    Amulet,
}

#[error_code]
pub enum EnhanceError {
    #[msg("Type cosplay blocked: different equipment types required")]
    CosplayBlocked,
}
