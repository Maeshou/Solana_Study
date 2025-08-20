// 7) duration_estimator: 推定日数（少数の分岐のみ）
use anchor_lang::prelude::*;

declare_id!("Dur4t10n7777777777777777777777777777777");

#[program]
pub mod duration_estimator {
    use super::*;

    pub fn estimate(ctx: Context<Estimate>, tier: TournamentTier) -> Result<()> {
        let mut days = 5u32;
        if is_grandmaster(tier) { days = 14; }
        if is_master(tier) { days = 10; }
        if is_diamond(tier) { days = 7; }
        ctx.accounts.duration.estimated_days = days;
        Ok(())
    }

    fn is_diamond(t: TournamentTier) -> bool { if let TournamentTier::Diamond = t { return true } false }
    fn is_master(t: TournamentTier) -> bool { if let TournamentTier::Master = t { return true } false }
    fn is_grandmaster(t: TournamentTier) -> bool { if let TournamentTier::Grandmaster = t { return true } false }
}

#[derive(Accounts)]
pub struct Estimate<'info> {
    #[account(
        init,
        payer = planner,
        space = 8 + DurationBook::LEN,
        seeds = [b"duration", planner.key().as_ref()],
        bump
    )]
    pub duration: Account<'info, DurationBook>,
    #[account(mut)]
    pub planner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DurationBook { pub estimated_days: u32 }
impl DurationBook { pub const LEN: usize = 4; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum TournamentTier { Bronze, Silver, Gold, Platinum, Diamond, Master, Grandmaster }
