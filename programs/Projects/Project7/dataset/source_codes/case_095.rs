// 04. NFTクラフト用素材消費
use anchor_lang::prelude::*;
use anchor_spl::token::*;

declare_id!("5hF4G3e2D1c0B9a8W7v6U5t4S3R2Q1P0o9N8m7L6K5J4I3H2G1F0E9D8C7B6");

#[program]
pub mod nft_crafter {
    use super::*;

    pub fn initialize_recipe(ctx: Context<InitializeRecipe>, recipe_name: String, required_materials: u64) -> Result<()> {
        let recipe = &mut ctx.accounts.crafting_recipe;
        recipe.recipe_creator = ctx.accounts.creator.key();
        recipe.recipe_name = recipe_name;
        recipe.required_materials = required_materials;
        recipe.material_mint = ctx.accounts.material_mint.key();
        Ok(())
    }

    pub fn craft_nft(ctx: Context<CraftNft>) -> Result<()> {
        let recipe = &ctx.accounts.crafting_recipe;
        let player_materials = ctx.accounts.player_material_account.amount;

        if player_materials < recipe.required_materials {
            return Err(ErrorCode::InsufficientMaterials.into());
        }

        let burn_amount = recipe.required_materials;
        let mut crafted_count = 0;
        let mut remaining_materials = player_materials;

        while remaining_materials >= burn_amount {
            let burn_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Burn {
                    mint: ctx.accounts.material_mint.to_account_info(),
                    from: ctx.accounts.player_material_account.to_account_info(),
                    authority: ctx.accounts.player.to_account_info(),
                },
            );
            token::burn(burn_ctx, burn_amount)?;
            remaining_materials -= burn_amount;
            crafted_count += 1;
        }

        if crafted_count > 0 {
            // Logic to mint NFT would go here, which is another CPI
        } else {
            return Err(ErrorCode::InsufficientMaterials.into());
        }

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(recipe_name: String, required_materials: u64)]
pub struct InitializeRecipe<'info> {
    #[account(init, payer = creator, space = 8 + 32 + 4 + recipe_name.len() + 8 + 32)]
    pub crafting_recipe: Account<'info, CraftingRecipe>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub material_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CraftNft<'info> {
    #[account(has_one = material_mint)]
    pub crafting_recipe: Account<'info, CraftingRecipe>,
    #[account(mut, has_one = owner)]
    pub player_material_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub material_mint: Account<'info, Mint>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct CraftingRecipe {
    pub recipe_creator: Pubkey,
    pub recipe_name: String,
    pub required_materials: u64,
    pub material_mint: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Not enough materials to craft the item.")]
    InsufficientMaterials,
}