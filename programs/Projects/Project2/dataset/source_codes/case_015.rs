// =============================================================================
// 15. Freelancer Job Board Platform
// =============================================================================
#[program]
pub mod secure_freelancer {
    use super::*;

    pub fn create_job_posting(ctx: Context<CreateJobPosting>, title: String, description: String, budget: u64) -> Result<()> {
        let job_posting = &mut ctx.accounts.job_posting;
        job_posting.client = ctx.accounts.client.key();
        job_posting.title = title;
        job_posting.description = description;
        job_posting.budget = budget;
        job_posting.is_open = true;
        job_posting.freelancer = None;
        job_posting.created_at = Clock::get()?.unix_timestamp;
        job_posting.bump = *ctx.bumps.get("job_posting").unwrap();
        Ok(())
    }

    pub fn submit_proposal(ctx: Context<SubmitProposal>, bid_amount: u64, proposal_text: String) -> Result<()> {
        let job_posting = &ctx.accounts.job_posting;
        let proposal = &mut ctx.accounts.proposal;
        
        require!(job_posting.is_open, FreelancerError::JobNotOpen);
        
        proposal.job_posting = job_posting.key();
        proposal.freelancer = ctx.accounts.freelancer.key();
        proposal.bid_amount = bid_amount;
        proposal.proposal_text = proposal_text;
        proposal.status = ProposalStatus::Pending;
        proposal.submitted_at = Clock::get()?.unix_timestamp;
        proposal.bump = *ctx.bumps.get("proposal").unwrap();
        
        Ok(())
    }

    pub fn accept_proposal(ctx: Context<AcceptProposal>) -> Result<()> {
        let job_posting = &mut ctx.accounts.job_posting;
        let proposal = &mut ctx.accounts.proposal;
        
        require!(job_posting.is_open, FreelancerError::JobNotOpen);
        require!(matches!(proposal.status, ProposalStatus::Pending), FreelancerError::ProposalNotPending);
        
        proposal.status = ProposalStatus::Accepted;
        job_posting.freelancer = Some(proposal.freelancer);
        job_posting.is_open = false;
        
        Ok(())
    }

    pub fn complete_job(ctx: Context<CompleteJob>) -> Result<()> {
        let job_posting = &mut ctx.accounts.job_posting;
        let proposal = &ctx.accounts.proposal;
        
        require!(!job_posting.is_open, FreelancerError::JobStillOpen);
        require!(matches!(proposal.status, ProposalStatus::Accepted), FreelancerError::ProposalNotAccepted);
        
        // Transfer payment to freelancer
        **ctx.accounts.client.lamports.borrow_mut() -= proposal.bid_amount;
        **ctx.accounts.freelancer.lamports.borrow_mut() += proposal.bid_amount;
        
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum ProposalStatus {
    Pending,
    Accepted,
    Rejected,
}

#[account]
pub struct JobPosting {
    pub client: Pubkey,
    pub title: String,
    pub description: String,
    pub budget: u64,
    pub is_open: bool,
    pub freelancer: Option<Pubkey>,
    pub created_at: i64,
    pub bump: u8,
}

#[account]
pub struct Proposal {
    pub job_posting: Pubkey,
    pub freelancer: Pubkey,
    pub bid_amount: u64,
    pub proposal_text: String,
    pub status: ProposalStatus,
    pub submitted_at: i64,
    pub bump: u8,
}

#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct CreateJobPosting<'info> {
    #[account(
        init,
        payer = client,
        space = 8 + 32 + 4 + title.len() + 4 + description.len() + 8 + 1 + 33 + 8 + 1,
        seeds = [b"job", client.key().as_ref(), title.as_bytes()],
        bump
    )]
    pub job_posting: Account<'info, JobPosting>,
    
    #[account(mut)]
    pub client: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(bid_amount: u64, proposal_text: String)]
pub struct SubmitProposal<'info> {
    #[account(
        seeds = [b"job", job_posting.client.as_ref(), job_posting.title.as_bytes()],
        bump = job_posting.bump
    )]
    pub job_posting: Account<'info, JobPosting>,
    
    #[account(
        init,
        payer = freelancer,
        space = 8 + 32 + 32 + 8 + 4 + proposal_text.len() + 32 + 8 + 1,
        seeds = [b"proposal", job_posting.key().as_ref(), freelancer.key().as_ref()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,
    
    #[account(mut)]
    pub freelancer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AcceptProposal<'info> {
    #[account(
        mut,
        seeds = [b"job", client.key().as_ref(), job_posting.title.as_bytes()],
        bump = job_posting.bump,
        constraint = job_posting.client == client.key()
    )]
    pub job_posting: Account<'info, JobPosting>,
    
    #[account(
        mut,
        seeds = [b"proposal", job_posting.key().as_ref(), proposal.freelancer.as_ref()],
        bump = proposal.bump,
        constraint = proposal.job_posting == job_posting.key()
    )]
    pub proposal: Account<'info, Proposal>,
    
    pub client: Signer<'info>,
}

#[derive(Accounts)]
pub struct CompleteJob<'info> {
    #[account(
        seeds = [b"job", client.key().as_ref(), job_posting.title.as_bytes()],
        bump = job_posting.bump,
        constraint = job_posting.client == client.key()
    )]
    pub job_posting: Account<'info, JobPosting>,
    
    #[account(
        seeds = [b"proposal", job_posting.key().as_ref(), freelancer.key().as_ref()],
        bump = proposal.bump,
        constraint = proposal.freelancer == freelancer.key()
    )]
    pub proposal: Account<'info, Proposal>,
    
    #[account(mut)]
    pub client: Signer<'info>,
    
    #[account(mut)]
    pub freelancer: Signer<'info>,
}

#[error_code]
pub enum FreelancerError {
    #[msg("Job posting is not open")]
    JobNotOpen,
    #[msg("Proposal is not pending")]
    ProposalNotPending,
    #[msg("Job is still open")]
    JobStillOpen,
    #[msg("Proposal is not accepted")]
    ProposalNotAccepted,
}
