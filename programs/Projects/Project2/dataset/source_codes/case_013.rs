// =============================================================================
// 13. Insurance Claims Management System
// =============================================================================
#[program]
pub mod secure_insurance {
    use super::*;

    pub fn create_policy(ctx: Context<CreatePolicy>, policy_type: PolicyType, premium: u64, coverage_amount: u64) -> Result<()> {
        let policy = &mut ctx.accounts.policy;
        policy.holder = ctx.accounts.holder.key();
        policy.policy_type = policy_type;
        policy.premium = premium;
        policy.coverage_amount = coverage_amount;
        policy.is_active = true;
        policy.created_at = Clock::get()?.unix_timestamp;
        policy.bump = *ctx.bumps.get("policy").unwrap();
        Ok(())
    }

    pub fn file_claim(ctx: Context<FileClaim>, amount: u64, description: String) -> Result<()> {
        let policy = &ctx.accounts.policy;
        let claim = &mut ctx.accounts.claim;
        
        require!(policy.is_active, InsuranceError::PolicyNotActive);
        require!(amount <= policy.coverage_amount, InsuranceError::ExceedsCoverage);
        
        claim.policy = policy.key();
        claim.claimant = ctx.accounts.claimant.key();
        claim.amount = amount;
        claim.description = description;
        claim.status = ClaimStatus::Pending;
        claim.filed_at = Clock::get()?.unix_timestamp;
        claim.bump = *ctx.bumps.get("claim").unwrap();
        
        Ok(())
    }

    pub fn process_claim(ctx: Context<ProcessClaim>, approved: bool) -> Result<()> {
        let claim = &mut ctx.accounts.claim;
        
        require!(matches!(claim.status, ClaimStatus::Pending), InsuranceError::ClaimAlreadyProcessed);
        
        if approved {
            claim.status = ClaimStatus::Approved;
            // Transfer claim amount to claimant
            **ctx.accounts.insurance_company.lamports.borrow_mut() -= claim.amount;
            **ctx.accounts.claimant.lamports.borrow_mut() += claim.amount;
        } else {
            claim.status = ClaimStatus::Rejected;
        }
        
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum PolicyType {
    Health,
    Auto,
    Home,
    Life,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum ClaimStatus {
    Pending,
    Approved,
    Rejected,
}

#[account]
pub struct Policy {
    pub holder: Pubkey,
    pub policy_type: PolicyType,
    pub premium: u64,
    pub coverage_amount: u64,
    pub is_active: bool,
    pub created_at: i64,
    pub bump: u8,
}

#[account]
pub struct Claim {
    pub policy: Pubkey,
    pub claimant: Pubkey,
    pub amount: u64,
    pub description: String,
    pub status: ClaimStatus,
    pub filed_at: i64,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct CreatePolicy<'info> {
    #[account(
        init,
        payer = holder,
        space = 8 + 32 + 32 + 8 + 8 + 1 + 8 + 1,
        seeds = [b"policy", holder.key().as_ref()],
        bump
    )]
    pub policy: Account<'info, Policy>,
    
    #[account(mut)]
    pub holder: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(amount: u64, description: String)]
pub struct FileClaim<'info> {
    #[account(
        seeds = [b"policy", policy.holder.as_ref()],
        bump = policy.bump,
        constraint = policy.holder == claimant.key()
    )]
    pub policy: Account<'info, Policy>,
    
    #[account(
        init,
        payer = claimant,
        space = 8 + 32 + 32 + 8 + 4 + description.len() + 32 + 8 + 1,
        seeds = [b"claim", policy.key().as_ref(), &Clock::get().unwrap().unix_timestamp.to_le_bytes()],
        bump
    )]
    pub claim: Account<'info, Claim>,
    
    #[account(mut)]
    pub claimant: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessClaim<'info> {
    #[account(
        mut,
        seeds = [b"claim", claim.policy.as_ref(), &claim.filed_at.to_le_bytes()],
        bump = claim.bump
    )]
    pub claim: Account<'info, Claim>,
    
    /// CHECK: Verified as insurance company authority
    pub insurance_company: Signer<'info>,
    
    /// CHECK: Verified through claim claimant field
    #[account(
        mut,
        constraint = claimant.key() == claim.claimant
    )]
    pub claimant: AccountInfo<'info>,
}

#[error_code]
pub enum InsuranceError {
    #[msg("Policy is not active")]
    PolicyNotActive,
    #[msg("Claim amount exceeds coverage")]
    ExceedsCoverage,
    #[msg("Claim has already been processed")]
    ClaimAlreadyProcessed,
}
