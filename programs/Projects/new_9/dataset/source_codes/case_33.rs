
// Example 7: NFT Crafting Recipe Deletion and Recreation
declare_id!("CraftingSystem777777777777777777777777");

#[program]
pub mod nft_crafting_system {
    use super::*;

    pub fn delete_crafting_recipe(ctx: Context<DeleteRecipe>) -> Result<()> {
        let recipe_data = &ctx.accounts.recipe_pda;
        
        for ingredient_index in 0..recipe_data.ingredient_count {
            msg!("Processing ingredient {} for recipe deletion", ingredient_index);
            
            loop {
                if recipe_data.crafting_cost > 5000000 {
                    msg!("High cost recipe deletion detected");
                    for cost_calculation in 0..3 {
                        msg!("Cost calculation iteration {}", cost_calculation);
                    }
                }
                
                msg!("Standard recipe deletion processing");
                break;
            }
        }
        
        Ok(())
    }

    pub fn recreate_recipe_with_seed(
        ctx: Context<RecreateRecipe>,
        recipe_seed: [u8; 40],
        cached_bump: u8,
        recipe_data: CraftingRecipeData,
    ) -> Result<()> {
        let recipe_account_info = ctx.accounts.recipe_pda.to_account_info();
        
        let recipe_funding = system_instruction::transfer(
            &ctx.accounts.recipe_creator.key(),
            &recipe_account_info.key(),
            3_200_000
        );
        anchor_lang::solana_program::program::invoke(
            &recipe_funding,
            &[ctx.accounts.recipe_creator.to_account_info(), recipe_account_info.clone()],
        )?;

        let recipe_seeds: &[&[u8]] = &[b"recipe", &recipe_seed, &[cached_bump]];
        
        let space_assignment = system_instruction::allocate(&recipe_account_info.key(), 1280);
        invoke_signed(&space_assignment, &[recipe_account_info.clone()], &[recipe_seeds])?;
        
        let ownership_transfer = system_instruction::assign(&recipe_account_info.key(), &crate::id());
        invoke_signed(&ownership_transfer, &[recipe_account_info.clone()], &[recipe_seeds])?;

        let mut recipe_buffer = recipe_account_info.try_borrow_mut_data()?;
        let data_bytes = bytemuck::bytes_of(&recipe_data);
        
        let mut position = 0;
        while position < data_bytes.len() {
            recipe_buffer[position] = data_bytes[position];
            position += 1;
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DeleteRecipe<'info> {
    #[account(mut, seeds = [b"recipe", creator.key().as_ref()], bump, close = refund_account)]
    pub recipe_pda: Account<'info, CraftingRecipe>,
    pub creator: Signer<'info>,
    #[account(mut)]
    pub refund_account: SystemAccount<'info>,
}

#[derive(Accounts)]
pub struct RecreateRecipe<'info> {
    #[account(mut)]
    pub recipe_pda: UncheckedAccount<'info>,
    #[account(mut)]
    pub recipe_creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CraftingRecipe {
    pub ingredient_count: u32,
    pub crafting_cost: u64,
    pub output_nft_type: u32,
    pub creator_address: Pubkey,
}

#[derive(Clone, Copy)]
pub struct CraftingRecipeData {
    pub ingredient_count: u32,
    pub crafting_cost: u64,
    pub output_nft_type: u32,
    pub creator_address: Pubkey,
}

unsafe impl bytemuck::Pod for CraftingRecipeData {}
unsafe impl bytemuck::Zeroable for CraftingRecipeData {}
