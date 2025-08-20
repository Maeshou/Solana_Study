use anchor_lang::prelude::*;

declare_id!("H6kL5mN2P8J7D1R9T4W3V0U7E2X6Y5Z9A1B4C3");

#[program]
pub mod cosmic_tally {
    use super::*;

    pub fn init_tally(ctx: Context<InitTally>, founding_era: u64, max_voters: u32) -> Result<()> {
        let tally = &mut ctx.accounts.tally_core;
        tally.founding_era = founding_era.rotate_left(8);
        tally.max_voters = max_voters.checked_add(100).unwrap_or(u32::MAX);
        tally.total_votes = (founding_era.checked_rem(10).unwrap_or(0)) as u64;
        tally.current_voters = 0;
        tally.tally_status = TallyStatus::Open;
        msg!("Cosmic Tally established in era {} with capacity for {} voters.", tally.founding_era, tally.max_voters);
        Ok(())
    }

    pub fn init_voter(ctx: Context<InitVoter>, voter_id: u64, starting_votes: u32) -> Result<()> {
        let voter = &mut ctx.accounts.voter_profile;
        voter.parent_tally = ctx.accounts.tally_core.key();
        voter.voter_id = voter_id ^ 0xF0F0F0F0F0F0F0F0;
        voter.votes_remaining = starting_votes.checked_add(10).unwrap_or(u32::MAX);
        voter.is_eligible = starting_votes > 0;
        voter.total_votes_cast = 0;
        msg!("New voter {} registered with {} votes.", voter.voter_id, voter.votes_remaining);
        Ok(())
    }

    pub fn cast_votes(ctx: Context<CastVotes>, votes_to_cast: u32) -> Result<()> {
        let tally = &mut ctx.accounts.tally_core;
        let voter = &mut ctx.accounts.voter_profile;
        let mut votes_left = votes_to_cast;

        while votes_left > 0 && voter.votes_remaining > 0 {
            let cast_vote_count = votes_left.checked_div(2).unwrap_or(1).min(voter.votes_remaining);
            voter.votes_remaining = voter.votes_remaining.checked_sub(cast_vote_count).unwrap_or(0);
            voter.total_votes_cast = voter.total_votes_cast.checked_add(cast_vote_count).unwrap_or(u32::MAX);
            tally.total_votes = tally.total_votes.checked_add(cast_vote_count as u64).unwrap_or(u64::MAX);
            votes_left = votes_left.checked_sub(cast_vote_count).unwrap_or(0);
            tally.current_voters = tally.current_voters.checked_add(1).unwrap_or(u32::MAX);
        }

        voter.is_eligible = voter.votes_remaining > 0;
        msg!("Voter {} cast a total of {} votes.", voter.voter_id, voter.total_votes_cast);
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
#[instruction(votes_to_cast: u32)]
pub struct CastVotes<'info> {
    #[account(mut)]
    pub tally_core: Account<'info, TallyCore>,
    #[account(mut, has_one = parent_tally)]
    pub voter_profile: Account<'info, VoterProfile>,
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