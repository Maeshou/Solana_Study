// =============================================================================
// 3. Voting System with Proper Owner Validation
// =============================================================================
#[program]
pub mod secure_voting {
    use super::*;

    pub fn create_proposal(ctx: Context<CreateProposal>, description: String) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.creator = ctx.accounts.creator.key();
        proposal.description = description;
        proposal.yes_votes = 0;
        proposal.no_votes = 0;
        proposal.bump = *ctx.bumps.get("proposal").unwrap();
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, vote_yes: bool) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        
        if vote_yes {
            proposal.yes_votes += 1;
        } else {
            proposal.no_votes += 1;
        }
        Ok(())
    }
}

#[account]
pub struct Proposal {
    pub creator: Pubkey,
    pub description: String,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub bump: u8,
}

#[derive(Accounts)]
#[instruction(description: String)]
pub struct CreateProposal<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + 32 + 4 + description.len() + 8 + 8 + 1,
        seeds = [b"proposal", creator.key().as_ref()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,
    
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(
        mut,
        constraint = proposal.creator != voter.key() @ VotingError::CreatorCannotVote
    )]
    pub proposal: Account<'info, Proposal>,
    
    pub voter: Signer<'info>,
}

#[error_code]
pub enum VotingError {
    #[msg("Creator cannot vote on their own proposal")]
    CreatorCannotVote,
}