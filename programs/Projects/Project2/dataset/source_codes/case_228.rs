use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("AlchRec7777777777777777777777777777777777");

#[program]
pub mod alchemy {
    use super::*;

    pub fn brew(
        ctx: Context<Brew>,
        recipe_id: u64,
    ) -> Result<()> {
        let r = &mut ctx.accounts.recipes;
        if r.known.contains(&recipe_id) {
            // 既知レシピ：作成成功
            *r.stock.entry(recipe_id).or_insert(0) += 1;
            r.brew_count = r.brew_count.saturating_add(1);
        } else {
            // 未知レシピ：失敗扱い
            r.unknown_attempts = r.unknown_attempts.saturating_add(1);
            r.waste_materials = r.waste_materials.saturating_add(1);
            r.error_map
                .entry(recipe_id)
                .or_insert_with(|| "Recipe unknown".to_string());
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Brew<'info> {
    #[account(mut)]
    pub recipes: Account<'info, RecipeData>,
}

#[account]
pub struct RecipeData {
    pub known: Vec<u64>,
    pub stock: BTreeMap<u64, u64>,
    pub brew_count: u64,
    pub unknown_attempts: u64,
    pub waste_materials: u64,
    pub error_map: BTreeMap<u64, String>,
}
