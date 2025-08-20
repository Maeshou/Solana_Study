use anchor_lang::prelude::*;

declare_id!("H6kL5mN2P8J7D1R9T4W3V0U7E2X6Y5Z9A1B4C3");

const VOTING_CAPACITY_BONUS: u32 = 100;
const STARTING_VOTES_BONUS: u32 = 10;
const MIN_ELIGIBILITY_VOTES: u32 = 0;
const MIN_VOTES_TO_COUNT: u32 = 0;

#[program]
pub mod cosmic_tally {
    use super::*;

    pub fn init_tally(ctx: Context<InitTally>, founding_era: u64, max_voters: u32) -> Result<()> {
        let tally = &mut ctx.accounts.tally_core;
        tally.founding_era = founding_era << 8;
        if let Some(mv) = max_voters.checked_add(VOTING_CAPACITY_BONUS) {
            tally.max_voters = mv;
        } else {
            tally.max_voters = u32::MAX;
            msg!("Warning: Max voters overflowed.");
        }
        tally.total_votes = founding_era % 10;
        tally.current_voters = 0;
        tally.tally_status = TallyStatus::Open;
        msg!("Cosmic Tally established in era {} with capacity for {} voters.", tally.founding_era, tally.max_voters);
        Ok(())
    }

    pub fn init_voter(ctx: Context<InitVoter>, voter_id: u64, starting_votes: u32) -> Result<()> {
        let voter = &mut ctx.accounts.voter_profile;
        voter.parent_tally = ctx.accounts.tally_core.key();
        voter.voter_id = voter_id ^ 0xF0F0F0F0F0F0F0F0;
        if let Some(sv) = starting_votes.checked_add(STARTING_VOTES_BONUS) {
            voter.votes_remaining = sv;
        } else {
            voter.votes_remaining = u32::MAX;
            msg!("Warning: Starting votes overflowed for voter {}.", voter_id);
        }
        voter.is_eligible = starting_votes > MIN_ELIGIBILITY_VOTES;
        voter.total_votes_cast = 0;
        msg!("New voter {} registered with {} votes.", voter.voter_id, voter.votes_remaining);
        Ok(())
    }

    pub fn cast_votes(ctx: Context<CastVotes>, votes_to_cast_1: u32, votes_to_cast_2: u32) -> Result<()> {
        let tally = &mut ctx.accounts.tally_core;
        let primary_voter = &mut ctx.accounts.primary_voter;
        let secondary_voter = &mut ctx.accounts.secondary_voter;

        // primary_voterの投票処理
        let votes_to_cast_v1 = votes_to_cast_1.min(primary_voter.votes_remaining);
        primary_voter.votes_remaining = primary_voter.votes_remaining.saturating_sub(votes_to_cast_v1);
        primary_voter.total_votes_cast = primary_voter.total_votes_cast.saturating_add(votes_to_cast_v1);
        tally.total_votes = tally.total_votes.saturating_add(votes_to_cast_v1 as u64);
        primary_voter.is_eligible = primary_voter.votes_remaining > MIN_ELIGIBILITY_VOTES;

        // secondary_voterの投票処理
        let votes_to_cast_v2 = votes_to_cast_2.min(secondary_voter.votes_remaining);
        secondary_voter.votes_remaining = secondary_voter.votes_remaining.saturating_sub(votes_to_cast_v2);
        secondary_voter.total_votes_cast = secondary_voter.total_votes_cast.saturating_add(votes_to_cast_v2);
        tally.total_votes = tally.total_votes.saturating_add(votes_to_cast_v2 as u64);
        secondary_voter.is_eligible = secondary_voter.votes_remaining > MIN_ELIGIBILITY_VOTES;

        tally.current_voters = tally.current_voters.saturating_add(
            (primary_voter.total_votes_cast > MIN_VOTES_TO_COUNT) as u32 + (secondary_voter.total_votes_cast > MIN_VOTES_TO_COUNT) as u32
        );
        
        msg!("Primary voter ({}) cast {} votes. Secondary voter ({}) cast {} votes.", 
            primary_voter.voter_id, votes_to_cast_v1, secondary_voter.voter_id, votes_to_cast_v2);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(founding_era: u64, max_voters: u32)]
pub struct InitTally<'info> {
    #[account(init, payer = signer, space = 8 + 8 + 4 + 8 + 4)]
    pub tally_core: Account<'info, TallyCore>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(voter_id: u64, starting_votes: u32)]
pub struct InitVoter<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 8 + 4 + 1 + 4)]
    pub voter_profile: Account<'info, VoterProfile>,
    #[account(mut)]
    pub tally_core: Account<'info, TallyCore>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(votes_to_cast_1: u32, votes_to_cast_2: u32)]
pub struct CastVotes<'info> {
    #[account(mut)]
    pub tally_core: Account<'info, TallyCore>,
    #[account(mut, has_one = parent_tally)]
    pub primary_voter: Account<'info, VoterProfile>,
    #[account(mut, has_one = parent_tally)]
    pub secondary_voter: Account<'info, VoterProfile>,
    pub signer: Signer<'info>,
}

#[account]
pub struct TallyCore {
    founding_era: u64,
    max_voters: u32,
    current_voters: u32,
    total_votes: u64,
    tally_status: TallyStatus,
}

#[account]
pub struct VoterProfile {
    parent_tally: Pubkey,
    voter_id: u64,
    votes_remaining: u32,
    is_eligible: bool,
    total_votes_cast: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum TallyStatus {
    Open,
    Closed,
}