// 7. DAO Governance with Proposal System
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};

declare_id!("DAOGovernance111111111111111111111111111111111111");

#[program]
pub mod dao_governance {
    use super::*;
    
    pub fn initialize_dao(ctx: Context<InitializeDAO>, voting_period: i64, min_tokens: u64) -> Result<()> {
        let dao = &mut ctx.accounts.dao;
        dao.governance_token = ctx.accounts.governance_token.key();
        dao.voting_period = voting_period;
        dao.min_tokens_to_propose = min_tokens;
        dao.proposal_count = 0;
        Ok(())
    }
    
    pub fn create_proposal(ctx: Context<CreateProposal>, title: String, description: String) -> Result<()> {
        require!(ctx.accounts.proposer_token_account.amount >= ctx.accounts.dao.min_tokens_to_propose, DAOError::InsufficientTokens);
        
        let dao = &mut ctx.accounts.dao;
        let proposal = &mut ctx.accounts.proposal;
        
        proposal.dao = dao.key();
        proposal.proposer = ctx.accounts.proposer.key();
        proposal.title = title;
        proposal.description = description;
        proposal.yes_votes = 0;
        proposal.no_votes = 0;
        proposal.end_time = Clock::get()?.unix_timestamp + dao.voting_period;
        proposal.executed = false;
        
        dao.proposal_count += 1;
        Ok(())
    }
    
    pub fn vote_on_proposal(ctx: Context<VoteOnProposal>, vote: bool, voting_power: u64) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        
        require!(Clock::get()?.unix_timestamp < proposal.end_time, DAOError::VotingPeriodEnded);
        require!(ctx.accounts.voter_token_account.amount >= voting_power, DAOError::InsufficientVotingPower);
        
        if vote {
            proposal.yes_votes += voting_power;
        } else {
            proposal.no_votes += voting_power;
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeDAO<'info> {
    #[account(init, payer = authority, space = 8 + 200)]
    pub dao: Account<'info, DAO>,
    pub governance_token: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub dao: Account<'info, DAO>,
    #[account(init, payer = proposer, space = 8 + 1000, seeds = [b"proposal", dao.key().as_ref(), &dao.proposal_count.to_le_bytes()], bump)]
    pub proposal: Account<'info, Proposal>,
    #[account(constraint = proposer_token_account.mint == dao.governance_token)]
    pub proposer_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub proposer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VoteOnProposal<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    #[account(constraint = voter_token_account.mint == dao.governance_token)]
    pub voter_token_account: Account<'info, TokenAccount>,
    pub dao: Account<'info, DAO>,
    pub voter: Signer<'info>,
}

#[account]
pub struct DAO {
    pub governance_token: Pubkey,
    pub voting_period: i64,
    pub min_tokens_to_propose: u64,
    pub proposal_count: u64,
}

#[account]
pub struct Proposal {
    pub dao: Pubkey,
    pub proposer: Pubkey,
    pub title: String,
    pub description: String,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub end_time: i64,
    pub executed: bool,
}

#[error_code]
pub enum DAOError {
    #[msg("Insufficient tokens to create proposal")]
    InsufficientTokens,
    #[msg("Voting period has ended")]
    VotingPeriodEnded,
    #[msg("Insufficient voting power")]
    InsufficientVotingPower,
}