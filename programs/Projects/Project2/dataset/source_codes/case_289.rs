use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("CraftRecipe8888888888888888888888888888888");

#[program]
pub mod craft_recipe {
    use super::*;

    pub fn craft(ctx: Context<Craft>, recipe: String) -> Result<()> {
        let cr = &mut ctx.accounts.data;
        if let Some(reqs) = cr.recipes.get(&recipe) {
            let mut possible = true;
            for (comp, &cnt) in reqs.iter() {
                if cr.inventory.get(comp).copied().unwrap_or(0) < cnt {
                    possible = false;
                    break;
                }
            }
            if possible {
                // 材料減少
                for (comp, &cnt) in reqs.iter() {
                    let stock = cr.inventory.get_mut(comp).unwrap();
                    *stock = stock.saturating_sub(cnt);
                }
                *cr.crafted.entry(recipe.clone()).or_insert(0) += 1;
            } else {
                cr.failed_attempts.saturating_add(1);
            }
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Craft<'info> {
    #[account(mut)]
    pub data: Account<'info, CraftData>,
}

#[account]
pub struct CraftData {
    pub recipes: BTreeMap<String, BTreeMap<String, u64>>,
    pub inventory: BTreeMap<String, u64>,
    pub crafted: BTreeMap<String, u64>,
    pub failed_attempts: u64,
}
