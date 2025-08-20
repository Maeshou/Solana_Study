// =====================================
// 5. Governance Voting Program
// =====================================
use anchor_lang::prelude::*;

declare_id!("55555555555555555555555555555555");

#[program]
pub mod secure_governance {
    use super::*;

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        title: String,
        description: String,
    ) -> Result<()> {
        // Governanceアカウントのowner check
        let governance_info = ctx.accounts.governance.to_account_info();
        require!(
            governance_info.owner == ctx.program_id,
            ErrorCode::InvalidGovernanceOwner
        );

        let proposal_info = ctx.accounts.proposal.to_account_info();
        require!(
            proposal_info.owner == ctx.program_id,
            ErrorCode::InvalidProposalOwner
        );

        let proposal = &mut ctx.accounts.proposal;
        proposal.governance = ctx.accounts.governance.key();
        proposal.proposer = ctx.accounts.proposer.key();
        proposal.title = title;
        proposal.description = description;
        proposal.votes_for = 0;
        proposal.votes_against = 0;
        proposal.created_at = Clock::get()?.unix_timestamp;
        proposal.is_active = true;

        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, vote_type: VoteType) -> Result<()> {
        // 複数のowner checkを実装
        require!(
            ctx.accounts.proposal.to_account_info().owner == ctx.program_id,
            ErrorCode::InvalidProposalOwner
        );
        require!(
            ctx.accounts.vote_record.to_account_info().owner == ctx.program_id,
            ErrorCode::InvalidVoteRecordOwner
        );

        let proposal = &mut ctx.accounts.proposal;
        require!(proposal.is_active, ErrorCode::ProposalNotActive);

        let vote_record = &mut ctx.accounts.vote_record;
        vote_record.voter = ctx.accounts.voter.key();
        vote_record.proposal = ctx.accounts.proposal.key();
        vote_record.vote_type = vote_type.clone();

        match vote_type {
            VoteType::For => proposal.votes_for += 1,
            VoteType::Against => proposal.votes_against += 1,
        }

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct CreateProposal<'info> {
    #[account(constraint = governance.to_account_info().owner == program_id)]
    pub governance: Account<'info, Governance>,
    #[account(
        init,
        payer = proposer,
        space = 8 + 32 + 32 + 4 + title.len() + 4 + description.len() + 8 + 8 + 8 + 1,
        constraint = proposal.to_account_info().owner == program_id
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub proposer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(
        mut,
        constraint = proposal.to_account_info().owner == program_id
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(
        init,
        payer = voter,
        space = 8 + 32 + 32 + 1,
        seeds = [b"vote", proposal.key().as_ref(), voter.key().as_ref()],
        bump,
        constraint = vote_record.to_account_info().owner == program_id
    )]
    pub vote_record: Account<'info, VoteRecord>,
    #[account(mut)]
    pub voter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Governance {
    pub admin: Pubkey,
    pub proposal_count: u64,
}

#[account]
pub struct Proposal {
    pub governance: Pubkey,
    pub proposer: Pubkey,
    pub title: String,
    pub description: String,
    pub votes_for: u64,
    pub votes_against: u64,
    pub created_at: i64,
    pub is_active: bool,
}

#[account]
pub struct VoteRecord {
    pub voter: Pubkey,
    pub proposal: Pubkey,
    pub vote_type: VoteType,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum VoteType {
    For,
    Against,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid governance account owner")]
    InvalidGovernanceOwner,
    #[msg("Invalid proposal account owner")]
    InvalidProposalOwner,
    #[msg("Invalid vote record account owner")]
    InvalidVoteRecordOwner,
    #[msg("Proposal is not active")]
    ProposalNotActive,
}
