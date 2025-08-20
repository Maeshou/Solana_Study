// 5) reward_planner: ティア別報酬テーブル（分岐はここに集約）
use anchor_lang::prelude::*;

declare_id!("RwPl4n55555555555555555555555555555555");

#[program]
pub mod reward_planner {
    use super::*;

    pub fn plan(ctx: Context<Plan>, tier: TournamentTier, stake: u64, rating: u64) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let mut table: Vec<(u32, u64)> = vec![(1, 10_000), (2, 5_000), (3, 2_500), (4, 1_000)];
        if is_silver(tier) { table = vec![(1, 20_000), (2, 10_000), (3, 5_000), (4, 2_000)]; }
        if is_gold(tier) { table = vec![(1, 50_000), (2, 25_000), (3, 12_500), (4, 5_000)]; }
        if is_platinum(tier) { table = vec![(1, 100_000), (2, 50_000), (3, 25_000), (4, 10_000)]; }
        if is_diamond(tier) { table = vec![(1, 250_000), (2, 125_000), (3, 62_500), (4, 25_000)]; }
        if is_master(tier) { table = vec![(1, 500_000), (2, 250_000), (3, 125_000), (4, 50_000)]; }
        if is_grandmaster(tier) { table = vec![(1, 1_000_000), (2, 500_000), (3, 250_000), (4, 100_000)]; }

        let mut out: Vec<RewardRow> = Vec::new();
        let mut i = 0usize;
        while i < table.len() {
            let placement = table[i].0;
            let base = table[i].1;

            let mut tier_bonus = base / 15;
            if is_diamond(tier) { tier_bonus = base / 10; }
            if is_master(tier) { tier_bonus = base / 8; }
            if is_grandmaster(tier) { tier_bonus = base / 5; }

            let seasonal_phase = ((now % (86_400 * 90)) / (86_400 * 30)) as u32;
            let seasonal = 250u32 + seasonal_phase.saturating_mul(100);

            let mut points = 150u32;
            if placement == 1 { points = 500; }
            if placement == 2 { points = 350; }
            if placement == 3 { points = 250; }

            out.push(RewardRow {
                placement,
                monetary: base.saturating_add(stake / 10).saturating_add(tier_bonus),
                experience: (base / 100).saturating_add(rating / 10),
                ranking_points: points,
                seasonal_bonus: seasonal,
            });
            i = i.saturating_add(1);
        }

        ctx.accounts.rewards.rows = out;
        Ok(())
    }

    fn is_silver(t: TournamentTier) -> bool { if let TournamentTier::Silver = t { return true } false }
    fn is_gold(t: TournamentTier) -> bool { if let TournamentTier::Gold = t { return true } false }
    fn is_platinum(t: TournamentTier) -> bool { if let TournamentTier::Platinum = t { return true } false }
    fn is_diamond(t: TournamentTier) -> bool { if let TournamentTier::Diamond = t { return true } false }
    fn is_master(t: TournamentTier) -> bool { if let TournamentTier::Master = t { return true } false }
    fn is_grandmaster(t: TournamentTier) -> bool { if let TournamentTier::Grandmaster = t { return true } false }
}

#[derive(Accounts)]
pub struct Plan<'info> {
    #[account(
        init,
        payer = planner,
        space = 8 + RewardsTable::LEN,
        seeds = [b"rewards", planner.key().as_ref()],
        bump
    )]
    pub rewards: Account<'info, RewardsTable>,
    #[account(mut)]
    pub planner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RewardsTable {
    pub rows: Vec<RewardRow>,
}
impl RewardsTable { pub const LEN: usize = 4 + 16 * RewardRow::LEN; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RewardRow {
    pub placement: u32,
    pub monetary: u64,
    pub experience: u64,
    pub ranking_points: u32,
    pub seasonal_bonus: u32,
}
impl RewardRow { pub const LEN: usize = 4 + 8 + 8 + 4 + 4; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum TournamentTier { Bronze, Silver, Gold, Platinum, Diamond, Master, Grandmaster }
