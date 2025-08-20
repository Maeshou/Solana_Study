use anchor_lang::prelude::*;

declare_id!("OwnChkE8000000000000000000000000000000009");

#[program]
pub mod craft_recipes {
    pub fn add_recipe(
        ctx: Context<AddRecipe>,
        recipe: String,
    ) -> Result<()> {
        let rc = &mut ctx.accounts.recipes;
        // 属性レベルで crafter を検証
        rc.list.push(recipe.clone());
        rc.add_count = rc.add_count.saturating_add(1);

        // cache_store は unchecked
        ctx.accounts.cache_store.data.borrow_mut().extend_from_slice(recipe.as_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddRecipe<'info> {
    #[account(mut, has_one = crafter)]
    pub recipes: Account<'info, RecipeBook>,
    pub crafter: Signer<'info>,
    /// CHECK: キャッシュストア、所有者検証なし
    #[account(mut)]
    pub cache_store: AccountInfo<'info>,
}

#[account]
pub struct RecipeBook {
    pub crafter: Pubkey,
    pub list: Vec<String>,
    pub add_count: u64,
}
