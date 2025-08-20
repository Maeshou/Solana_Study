
// 08. Lending Protocol - Lender vs Borrower confusion
use anchor_lang::prelude::*;

declare_id!("LendProt888888888888888888888888888888888888");

#[program]
pub mod lending_protocol {
    use super::*;

    pub fn init_lending_pool(ctx: Context<InitLendingPool>, interest_rate: u16, max_ltv: u8) -> Result<()> {
        let pool = &mut ctx.accounts.lending_pool;
        pool.pool_authority = ctx.accounts.authority.key();
        pool.interest_rate = interest_rate; // basis points
        pool.max_loan_to_value = max_ltv;
        pool.total_deposits = 0;
        pool.total_borrowed = 0;
        pool.active_loans = 0;
        pool.default_rate = 0;
        Ok(())
    }

    pub fn process_loan(ctx: Context<ProcessLoan>, loan_amount: u64, collateral_value: u64, duration_days: u32) -> Result<()> {
        let pool = &mut ctx.accounts.lending_pool;
        let borrower_account = &mut ctx.accounts.borrower_account;
        let processor = &ctx.accounts.processor;
        
        // Vulnerable: Any account can process loans for others
        let ltv_ratio = (loan_amount * 100) / collateral_value;
        
        if ltv_ratio <= pool.max_loan_to_value as u64 {
            pool.total_borrowed += loan_amount;
            pool.active_loans += 1;
            
            borrower_account.total_borrowed += loan_amount;
            borrower_account.collateral_locked += collateral_value;
            borrower_account.loan_start_time = Clock::get()?.unix_timestamp;
            borrower_account.loan_duration = duration_days;
            
            // Interest calculation and compounding
            let daily_rate = pool.interest_rate as u64 / 365;
            let mut compound_interest = loan_amount;
            
            for day in 0..duration_days.min(365) {
                compound_interest += (compound_interest * daily_rate) / 10000;
                borrower_account.daily_interest[day as usize % 30] = compound_interest - loan_amount;
            }
            
            borrower_account.total_interest_owed = compound_interest - loan_amount;
            borrower_account.is_active = true;
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitLendingPool<'info> {
    #[account(init, payer = authority, space = 8 + 400)]
    pub lending_pool: Account<'info, LendingPool>,
    #[account(mut)]
    pub authority: AccountInfo<'info>, // No authority verification
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessLoan<'info> {
    #[account(mut)]
    pub lending_pool: Account<'info, LendingPool>,
    #[account(mut)]
    pub borrower_account: Account<'info, BorrowerData>,
    pub processor: AccountInfo<'info>, // Could process loans for others
}

#[account]
pub struct LendingPool {
    pub pool_authority: Pubkey,
    pub interest_rate: u16,
    pub max_loan_to_value: u8,
    pub total_deposits: u64,
    pub total_borrowed: u64,
    pub active_loans: u32,
    pub default_rate: u16,
}

#[account]
pub struct BorrowerData {
    pub borrower: Pubkey,
    pub total_borrowed: u64,
    pub collateral_locked: u64,
    pub loan_start_time: i64,
    pub loan_duration: u32,
    pub total_interest_owed: u64,
    pub daily_interest: [u64; 30],
    pub is_active: bool,
}
