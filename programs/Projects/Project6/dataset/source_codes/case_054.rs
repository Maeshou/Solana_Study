use anchor_lang::prelude::*;

declare_id!("ENCHT999999999999999999999999999999999999999");

#[program]
pub mod enchanting_program {
    use super::*;
    /// 武器のエンチャントを試みる
    pub fn enchant_weapon(ctx: Context<EnchantWeapon>) -> Result<()> {
        let weapon = &mut ctx.accounts.weapon_to_enchant;
        let scroll = &mut ctx.accounts.enchantment_scroll;
        let clock = Clock::get()?;

        let success_seed = (clock.slot % 100) as u32;
        let success_chance = 75 - (weapon.enchant_level * 5);
        let is_success = success_seed < success_chance;
        
        let success_increment = is_success as u32;
        weapon.enchant_level = weapon.enchant_level.saturating_add(success_increment);
        weapon.base_attack = weapon.base_attack.saturating_add(success_increment * 5);
        
        scroll.quantity = scroll.quantity.saturating_sub(1);

        msg!("Enchantment finished. Weapon level is now: {}", weapon.enchant_level);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct EnchantWeapon<'info> {
    #[account(mut, has_one = owner)]
    pub weapon_to_enchant: Account<'info, Weapon>,
    #[account(mut, constraint = enchantment_scroll.quantity > 0)]
    pub enchantment_scroll: Account<'info, Item>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Weapon {
    pub owner: Pubkey,
    pub enchant_level: u32,
    pub base_attack: u32,
}

#[account]
pub struct Item {
    pub owner: Pubkey,
    pub quantity: u32,
}