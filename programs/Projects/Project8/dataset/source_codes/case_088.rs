// 6) promo_builder: プレミアム有無でプロモ一覧を構築（bool分岐のみ）
use anchor_lang::prelude::*;

declare_id!("PromoBLD444444444444444444444444444444");

#[program]
pub mod promo_builder {
    use super::*;

    pub fn build_promotions(
        ctx: Context<BuildPromotions>,
        premium_features_enabled: bool,
    ) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let mut boosts: Vec<PromotionalBoost> = Vec::new();

        if premium_features_enabled {
            boosts.push(PromotionalBoost { boost_type: BoostType::FeaturedPlacement, duration_hours: 24, cost_paid: 200, activation_time: now });
            boosts.push(PromotionalBoost { boost_type: BoostType::HighlightListing, duration_hours: 48, cost_paid: 150, activation_time: now });
            boosts.push(PromotionalBoost { boost_type: BoostType::CrossCategoryPromotion, duration_hours: 72, cost_paid: 300, activation_time: now });
        }
        if !premium_features_enabled {
            boosts.push(PromotionalBoost { boost_type: BoostType::BasicVisibility, duration_hours: 12, cost_paid: 50, activation_time: now });
        }

        ctx.accounts.promo.promotional_boosts = boosts;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BuildPromotions<'info> {
    #[account(
        init,
        payer = marketer,
        space = 8 + PromoTable::LEN,
        seeds = [b"promo", marketer.key().as_ref()],
        bump
    )]
    pub promo: Account<'info, PromoTable>,
    #[account(mut)]
    pub marketer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PromoTable { pub promotional_boosts: Vec<PromotionalBoost> }
impl PromoTable { pub const LEN: usize = 4 + 8 * PromotionalBoost::LEN; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PromotionalBoost {
    pub boost_type: BoostType,
    pub duration_hours: u32,
    pub cost_paid: u64,
    pub activation_time: i64,
}
impl PromotionalBoost { pub const LEN: usize = 1 + 4 + 8 + 8; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum BoostType { BasicVisibility, FeaturedPlacement, HighlightListing, CrossCategoryPromotion }
