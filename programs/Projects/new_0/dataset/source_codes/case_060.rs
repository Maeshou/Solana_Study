use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfVoteSys");

#[program]
pub mod vote_system {
    use super::*;

    pub fn initialize_token(ctx: Context<InitializeToken>) -> Result<()> {
        let token = &mut ctx.accounts.voter_token;
        token.owner = ctx.accounts.owner.key();
        token.used  = false;
        Ok(())
    }

    pub fn cast_vote(ctx: Context<CastVote>, vote_id: u64) -> Result<()> {
        let token = &mut ctx.accounts.voter_token;
        let votes = &mut ctx.accounts.vote_counter;

        // 既に使われていればエラー
        require!(!token.used, VoteError::AlreadyVoted);

        token.used           = true;
        votes.candidate_id   = vote_id;
        votes.total          = votes
            .total
            .checked_add(1)
            .ok_or(VoteError::CountOverflow)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeToken<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 1,
        seeds = [b"token", owner.key().as_ref()],
        bump
    )]
    pub voter_token: Account<'info, VoterToken>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(
        mut,
        seeds = [b"token", owner.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub voter_token: Account<'info, VoterToken>,

    #[account(
        mut,
        seeds = [b"vote", &vote_id.to_le_bytes()],
        bump
    )]
    pub vote_counter: Account<'info, VoteCounter>,

    pub owner: Signer<'info>,
}

#[account]
pub struct VoterToken {
    pub owner: Pubkey,
    pub used:  bool,
}

#[account]
pub struct VoteCounter {
    pub candidate_id: u64,
    pub total:        u64,
}

#[error_code]
pub enum VoteError {
    #[msg("You have already cast your vote")]
    AlreadyVoted,
    #[msg("Vote count overflow")]
    CountOverflow,
}
