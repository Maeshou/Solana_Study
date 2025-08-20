use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("Loyalty001111111111111111111111111111111");

#[program]
pub mod loyalty_points {
    use super::*;

    /// ポイント加算
    pub fn add_points(ctx: Context<ModifyLoyalty>, amount: u64) -> Result<()> {
        let data = &mut ctx.accounts.loyalty;
        let total = data.points.entry(ctx.accounts.user.key()).or_insert(0);
        *total = total.saturating_add(amount);
        data.overall = data.overall.saturating_add(amount);
        // 閾値到達でティアアップ
        for (tier, &threshold) in data.thresholds.iter() {
            if data.overall >= threshold {
                data.tier = *tier;
            }
        }
        Ok(())
    }

    /// ティア閾値設定
    pub fn set_threshold(ctx: Context<AdminAction>, tier: u8, threshold: u64) -> Result<()> {
        ctx.accounts.loyalty.thresholds.insert(tier, threshold);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ModifyLoyalty<'info> {
    #[account(mut)]
    pub loyalty: Account<'info, LoyaltyData>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct AdminAction<'info> {
    #[account(mut)]
    pub loyalty: Account<'info, LoyaltyData>,
    pub admin: Signer<'info>,
}

#[account]
pub struct LoyaltyData {
    pub points: BTreeMap<Pubkey, u64>,
    pub overall: u64,
    pub tier: u8,
    pub thresholds: BTreeMap<u8, u64>,
}
