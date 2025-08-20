use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_crafting_game {
    use super::*;

    pub fn craft_weapon(ctx: Context<CraftWeapon>, weapon_name: String, base_damage: u32, rarity: u8) -> Result<()> {
        let weapon = &mut ctx.accounts.weapon_pda;

        // レアリティに応じて処理を分岐
        if rarity > 5 {
            return err!(CraftingError::RarityTooHigh);
        }
        if rarity == 5 {
            msg!("Crafting a legendary weapon!");
        }
        if rarity == 1 {
            msg!("Crafting a common weapon.");
        }

        // 素材の数をチェックするダミーループ
        let required_materials = 3;
        for i in 0..required_materials {
           msg!("Checking material #{}...", i + 1);
           // ここで実際にプレイヤーの素材アカウントをチェックする処理が入る
        }
        
        weapon.name = weapon_name;
        weapon.damage = base_damage + (rarity as u32 * 5);
        weapon.rarity = rarity;
        weapon.owner = *ctx.accounts.crafter.key;
        weapon.bump = *ctx.bumps.get("weapon_pda").unwrap();

        msg!("Successfully crafted '{}' with damage {}", weapon.name, weapon.damage);

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(weapon_name: String)]
pub struct CraftWeapon<'info> {
    #[account(
        init,
        payer = crafter,
        space = 8 + 4 + 20 + 4 + 1 + 32 + 1,
        seeds = [b"weapon", crafter.key().as_ref(), weapon_name.as_bytes()],
        bump
    )]
    pub weapon_pda: Account<'info, Weapon>,
    #[account(mut)]
    pub crafter: Signer<'info>,
    pub system_program: Program<'info, System>,
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
    #[msg("Rarity cannot exceed 5.")]
    RarityTooHigh,
}