// 2) category_rules: カテゴリ別の増額幅・特集可否・評判要件・倍率（match/else-if/&&なし）
use anchor_lang::prelude::*;

declare_id!("CatRul44444444444444444444444444444444");

#[program]
pub mod category_rules {
    use super::*;

    pub fn apply_category_rules(
        ctx: Context<ApplyCategoryRules>,
        starting_price: u64,
        category: ItemCategory,
    ) -> Result<()> {
        let tuning = &mut ctx.accounts.tuning;

        // デフォルト
        tuning.minimum_bid_increment = starting_price / 100;
        tuning.featured_listing = false;
        tuning.seller_reputation_requirement = 10;
        tuning.category_multiplier_pct = 50;

        // LegendaryWeapon
        if is_legendary_weapon(category) {
            tuning.minimum_bid_increment = starting_price / 20;
            tuning.featured_listing = true;
            tuning.seller_reputation_requirement = 100;
            tuning.category_multiplier_pct = 300;
        }
        // RareArmor
        if is_rare_armor(category) {
            tuning.minimum_bid_increment = starting_price / 25;
            tuning.featured_listing = true;
            tuning.seller_reputation_requirement = 75;
            tuning.category_multiplier_pct = 200;
        }
        // EpicAccessory
        if is_epic_accessory(category) {
            tuning.minimum_bid_increment = starting_price / 30;
            tuning.featured_listing = false;
            tuning.seller_reputation_requirement = 50;
            tuning.category_multiplier_pct = 250;
        }
        // CraftingMaterial
        if is_crafting_material(category) {
            tuning.minimum_bid_increment = starting_price / 50;
            tuning.featured_listing = false;
            tuning.seller_reputation_requirement = 25;
            tuning.category_multiplier_pct = 100;
        }
        // ConsumableItem
        if is_consumable_item(category) {
            tuning.minimum_bid_increment = starting_price / 100;
            tuning.featured_listing = false;
            tuning.seller_reputation_requirement = 10;
            tuning.category_multiplier_pct = 50;
        }

        Ok(())
    }

    fn is_legendary_weapon(c: ItemCategory) -> bool { if let ItemCategory::LegendaryWeapon = c { return true } false }
    fn is_rare_armor(c: ItemCategory) -> bool { if let ItemCategory::RareArmor = c { return true } false }
    fn is_epic_accessory(c: ItemCategory) -> bool { if let ItemCategory::EpicAccessory = c { return true } false }
    fn is_crafting_material(c: ItemCategory) -> bool { if let ItemCategory::CraftingMaterial = c { return true } false }
    fn is_consumable_item(c: ItemCategory) -> bool { if let ItemCategory::ConsumableItem = c { return true } false }
}

#[derive(Accounts)]
pub struct ApplyCategoryRules<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + CategoryTuning::LEN,
        seeds = [b"tuning", authority.key().as_ref()],
        bump
    )]
    pub tuning: Account<'info, CategoryTuning>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CategoryTuning {
    pub minimum_bid_increment: u64,
    pub featured_listing: bool,
    pub seller_reputation_requirement: u32,
    pub category_multiplier_pct: u32,
}
impl CategoryTuning { pub const LEN: usize = 8 + 1 + 4 + 4; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum ItemCategory { LegendaryWeapon, RareArmor, EpicAccessory, CraftingMaterial, ConsumableItem }
