use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock;

// (パターン4のdeclare_id, Weapon, CraftingErrorを流用)
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_crafting_game {
    use super::*;
    pub fn enhance_weapon(ctx: Context<EnhanceWeapon>) -> Result<()> {
        let weapon = &mut ctx.accounts.weapon_pda;

        let mut enhancement_level = 0;
        let max_attempts = 5;
        
        // 強化試行をシミュレートするループ
        loop {
            enhancement_level += 1;
            let clock = clock::Clock::get()?;
            let pseudo_random = (clock.unix_timestamp as u32).wrapping_add(weapon.damage) % 100;

            // 擬似的なランダム値で成功判定
            if pseudo_random > 50 {
                weapon.damage += 5;
                msg!("Enhancement success! Attempt {}. New damage: {}", enhancement_level, weapon.damage);
            }
            
            if pseudo_random <= 50 {
                msg!("Enhancement failed. Attempt {}.", enhancement_level);
            }

            if enhancement_level >= max_attempts {
                msg!("Reached max enhancement attempts.");
                break;
            }
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct EnhanceWeapon<'info> {
    #[account(
        mut,
        seeds = [b"weapon", owner.key().as_ref(), weapon_pda.name.as_bytes()],
        bump = weapon_pda.bump,
        has_one = owner
    )]
    pub weapon_pda: Account<'info, Weapon>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Weapon {
    pub name: String,
    pub damage: u32,
    pub rarity: u8,
    pub owner: Pubkey,
    pub bump: u8,
}

#[error_code]
pub enum CraftingError {
    RarityTooHigh,
}