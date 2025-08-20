// 6) bracket_builder: ブラケット希望（経験レベル分岐をこちらに）
use anchor_lang::prelude::*;

declare_id!("Br4ck3t66666666666666666666666666666666");

#[program]
pub mod bracket_builder {
    use super::*;

    pub fn build(ctx: Context<Build>, rating: u64, total_matches: u64) -> Result<()> {
        let slots = [TimeSlot::Morning, TimeSlot::Afternoon, TimeSlot::Evening, TimeSlot::Night];
        let mut out: Vec<BracketPref> = Vec::new();

        let mut level = ExperienceLevel::Novice;
        if total_matches >= 100 { level = ExperienceLevel::Intermediate; }
        if total_matches >= 500 { level = ExperienceLevel::Experienced; }
        if total_matches >= 1000 { level = ExperienceLevel::Veteran; }

        let mut i = 0usize;
        while i < slots.len() {
            out.push(BracketPref {
                slot: slots[i],
                min_rating: rating.saturating_sub(200),
                max_rating: rating.saturating_add(200),
                exp_level: level,
                format: MatchFormat::BestOfThree,
            });
            i = i.saturating_add(1);
        }

        ctx.accounts.brackets.prefs = out;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Build<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + Brackets::LEN,
        seeds = [b"brackets", user.key().as_ref()],
        bump
    )]
    pub brackets: Account<'info, Brackets>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Brackets { pub prefs: Vec<BracketPref> }
impl Brackets { pub const LEN: usize = 4 + 8 * BracketPref::LEN; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum TimeSlot { Morning, Afternoon, Evening, Night }
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum ExperienceLevel { Novice, Intermediate, Experienced, Veteran }
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum MatchFormat { BestOfOne, BestOfThree, BestOfFive }

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BracketPref {
    pub slot: TimeSlot,
    pub min_rating: u64,
    pub max_rating: u64,
    pub exp_level: ExperienceLevel,
    pub format: MatchFormat,
}
impl BracketPref { pub const LEN: usize = 1 + 8 + 8 + 1 + 1; }
