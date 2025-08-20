// 6. Loan Portal
declare_id!("LP66666666666666666666666666666666");
use anchor_lang::prelude::*;

#[program]
pub mod loan_portal {
    use super::*;
    pub fn init_loan(ctx: Context<InitLoan>, amount: u64) -> Result<()> {
        ctx.accounts.loan_offer.amount = amount;
        ctx.accounts.loan_offer.interest = 5;
        ctx.accounts.loan_offer.owner = *ctx.accounts.payer.key;
        ctx.accounts.borrower_profile.credit_score = 700;
        ctx.accounts.borrower_profile.outstanding = amount;
        ctx.accounts.borrower_profile.verified = false;
        ctx.accounts.loan_stats.repayments = 0;
        ctx.accounts.loan_stats.late_count = 0;
        ctx.accounts.loan_stats.bump = *ctx.bumps.get("loan_offer").unwrap();
        Ok(())
    }
    pub fn adjust_loan(ctx: Context<AdjustLoan>, payment: u64) -> Result<()> {
        let mut paid = 0u64;
        let mut i = 0u64;
        while i < payment {
            paid += 1;
            i += 1;
        }
        if paid > ctx.accounts.loan_offer.amount {
            ctx.accounts.borrower_profile.outstanding -= ctx.accounts.loan_offer.amount;
            msg!("Overpayment");
            ctx.accounts.loan_stats.late_count += 1;
            ctx.accounts.loan_stats.repayments += 1;
        } else {
            ctx.accounts.borrower_profile.outstanding += paid;
            msg!("Payment applied");
            ctx.accounts.loan_stats.repayments += 1;
            ctx.accounts.loan_stats.late_count = 0;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitLoan<'info> {
    #[account(init, seeds = [b"offer", payer.key().as_ref()], bump, payer = payer, space = 8 + 8 + 4 + 32 + 1)]
    pub loan_offer: Account<'info, LoanOffer>,
    #[account(init, seeds = [b"profile", borrower.key().as_ref()], bump, payer = payer, space = 8 + 1 + 8 + 1)]
    pub borrower_profile: Account<'info, BorrowerProfile>,
    #[account(init, seeds = [b"stats", payer.key().as_ref()], bump, payer = payer, space = 8 + 8 + 4 + 1)]
    pub loan_stats: Account<'info, LoanStats>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub borrower: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AdjustLoan<'info> {
    #[account(mut, seeds = [b"offer", payer.key().as_ref()], bump)]
    pub loan_offer: Account<'info, LoanOffer>,
    #[account(mut, seeds = [b"profile", borrower.key().as_ref()], bump)]
    pub borrower_profile: Account<'info, BorrowerProfile>,
    #[account(mut, seeds = [b"stats", payer.key().as_ref()], bump)]
    pub loan_stats: Account<'info, LoanStats>,
}

#[account]
pub struct LoanOffer {
    pub amount: u64,
    pub interest: u32,
    pub owner: Pubkey,
}

#[account]
pub struct BorrowerProfile {
    pub credit_score: u8,
    pub outstanding: u64,
    pub verified: bool,
}

#[account]
pub struct LoanStats {
    pub repayments: u64,
    pub late_count: u32,
    pub bump: u8,
}


