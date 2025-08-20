// 2. Voting System with State Management
use anchor_lang::prelude::*;

declare_id!("VotingSystem1111111111111111111111111111111111");

#[program]
pub mod voting_system {
    use super::*;
    
    pub fn initialize_poll(ctx: Context<InitializePoll>, question: String, duration: i64) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        poll.creator = ctx.accounts.creator.key();
        poll.question = question;
        poll.yes_votes = 0;
        poll.no_votes = 0;
        poll.end_time = Clock::get()?.unix_timestamp + duration;
        poll.is_active = true;
        Ok(())
    }
    
    pub fn cast_vote(ctx: Context<CastVote>, vote_choice: bool) -> Result<()> {
        require!(ctx.accounts.poll.is_active, ErrorCode::PollInactive);
        require!(Clock::get()?.unix_timestamp < ctx.accounts.poll.end_time, ErrorCode::PollExpired);
        
        if vote_choice {
            ctx.accounts.poll.yes_votes += 1;
        } else {
            ctx.accounts.poll.no_votes += 1;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePoll<'info> {
    #[account(init, payer = creator, space = 8 + 300)]
    pub poll: Account<'info, Poll>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub poll: Account<'info, Poll>,
    pub voter: Signer<'info>,
}

#[account]
pub struct Poll {
    pub creator: Pubkey,
    pub question: String,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub end_time: i64,
    pub is_active: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Poll is not active")]
    PollInactive,
    #[msg("Poll has expired")]
    PollExpired,
}
