// 2) tier_rules: ティア要件チェック（分岐はここに寄せる）
use anchor_lang::prelude::*;

declare_id!("Ti3rRul22222222222222222222222222222222");

#[program]
pub mod tier_rules {
    use super::*;

    pub fn check_tier_requirements(
        ctx: Context<CheckTier>,
        total_matches: u64,
        win_rate: u32,
        current_skill_rating: u64,
        stake: u64,
        tier: TournamentTier,
    ) -> Result<()> {
        let mut min_matches = 10u64;
        let mut min_win = 30u32;
        let mut req_skill = 1000u64;
        let mut min_stake = 1000u64;

        if is_silver(tier) { min_matches = 50; min_win = 50; req_skill = 1500; min_stake = 2500; }
        if is_gold(tier) { min_matches = 100; min_win = 65; req_skill = 2000; min_stake = 5000; }
        if is_platinum(tier) { min_matches = 200; min_win = 75; req_skill = 2500; min_stake = 10_000; }
        if is_diamond(tier) { min_matches = 500; min_win = 80; req_skill = 3000; min_stake = 25_000; }
        if is_master(tier) { min_matches = 1000; min_win = 85; req_skill = 3500; min_stake = 50_000; }
        if is_grandmaster(tier) { min_matches = 2000; min_win = 90; req_skill = 4000; min_stake = 100_000; }

        require!(total_matches >= min_matches, TierError::InsufficientExperience);
        require!(win_rate >= min_win, TierError::WinRateTooLow);
        require!(stake >= min_stake, TierError::InsufficientStake);
        require!(current_skill_rating >= req_skill, TierError::SkillRatingTooLow);

        let gate = &mut ctx.accounts.tier_gate;
        gate.tier = tier;
        gate.required_skill = req_skill;
        gate.checked_at = Clock::get()?.unix_timestamp;
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
pub struct CheckTier<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + TierGate::LEN,
        seeds = [b"tier_gate", authority.key().as_ref()],
        bump
    )]
    pub tier_gate: Account<'info, TierGate>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct TierGate {
    pub tier: TournamentTier,
    pub required_skill: u64,
    pub checked_at: i64,
}
impl TierGate { pub const LEN: usize = 1 + 8 + 8; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum TournamentTier { Bronze, Silver, Gold, Platinum, Diamond, Master, Grandmaster }

#[error_code]
pub enum TierError {
    #[msg("Insufficient experience for tier.")] InsufficientExperience,
    #[msg("Win rate too low.")] WinRateTooLow,
    #[msg("Stake below minimum.")] InsufficientStake,
    #[msg("Skill rating below requirement.")] SkillRatingTooLow,
}
