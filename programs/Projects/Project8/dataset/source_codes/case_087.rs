// 5) fee_planner: 料金テーブル決定（閾値ifをここに集約、match/else-if/&&なし）
use anchor_lang::prelude::*;

declare_id!("FeePlan44444444444444444444444444444444");

#[program]
pub mod fee_planner {
    use super::*;

    pub fn decide_fees(
        ctx: Context<DecideFees>,
        estimated_value: u64,
    ) -> Result<()> {
        let fees = &mut ctx.accounts.fees;

        // デフォルト（Economy）
        fees.listing_fee = 50;
        fees.success_fee_percentage_bp = 500;
        fees.premium_features_enabled = false;
        fees.fee_structure = FeeStructure::Economy;

        // 10_000 以上（Basic）
        if estimated_value >= 10_000 {
            fees.listing_fee = 150;
            fees.success_fee_percentage_bp = 400;
            fees.premium_features_enabled = false;
            fees.fee_structure = FeeStructure::Basic;
        }
        // 50_000 以上（Standard）
        if estimated_value >= 50_000 {
            fees.listing_fee = 300;
            fees.success_fee_percentage_bp = 300;
            fees.premium_features_enabled = true;
            fees.fee_structure = FeeStructure::Standard;
        }
        // 100_000 以上（Premium）
        if estimated_value >= 100_000 {
            fees.listing_fee = 500;
            fees.success_fee_percentage_bp = 250;
            fees.premium_features_enabled = true;
            fees.fee_structure = FeeStructure::Premium;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct DecideFees<'info> {
    #[account(
        init,
        payer = seller,
        space = 8 + FeePlan::LEN,
        seeds = [b"fees", seller.key().as_ref()],
        bump
    )]
    pub fees: Account<'info, FeePlan>,
    #[account(mut)]
    pub seller: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct FeePlan {
    pub listing_fee: u64,
    pub success_fee_percentage_bp: u32, // basis points
    pub premium_features_enabled: bool,
    pub fee_structure: FeeStructure,
}
impl FeePlan { pub const LEN: usize = 8 + 4 + 1 + 1; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum FeeStructure { Economy, Basic, Standard, Premium }
