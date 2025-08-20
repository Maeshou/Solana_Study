use anchor_lang::prelude::*;

declare_id!("OwnChkB9000000000000000000000000000000009");

#[program]
pub mod recipe_craft {
    pub fn craft(ctx: Context<Craft>, recipe_id: u64) -> Result<()> {
        let rc = &mut ctx.accounts.recipedata;
        // has_one で crafter チェック済み
        rc.last_recipe = recipe_id;
        rc.craft_count = rc.craft_count.saturating_add(1);

        // recipe_cache は unchecked
        let mut cache = ctx.accounts.recipe_cache.data.borrow_mut();
        cache[0..8].copy_from_slice(&recipe_id.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Craft<'info> {
    #[account(mut, has_one = crafter)]
    pub recipedata: Account<'info, RecipeData>,
    pub crafter: Signer<'info>,
    /// CHECK: レシピキャッシュ、所有者検証なし
    #[account(mut)]
    pub recipe_cache: AccountInfo<'info>,
}

#[account]
pub struct RecipeData {
    pub crafter: Pubkey,
    pub last_recipe: u64,
    pub craft_count: u64,
}
