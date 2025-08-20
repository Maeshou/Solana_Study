use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

declare_id!("Secu41144444444444444444444444444444444");

#[program]
mod case_411 {
    use super::*;

    pub fn process_411(ctx: Context<Ctx411>) -> Result<()> {
        let acct_a = ctx.accounts.a.to_account_info();
        let acct_b = ctx.accounts.b.to_account_info();
        require!(acct_a.key() != acct_b.key(), ErrorCode::Same);
        let bal_a = **acct_a.try_borrow_lamports()?;
        let bal_b = **acct_b.try_borrow_lamports()?;
        let now = Clock::get()?.unix_timestamp;
        let sum = bal_a.checked_add(bal_b).unwrap();
        let avg = sum.checked_div(2).unwrap_or(0);
        let scaled = avg.checked_div(2).unwrap_or(0);
        msg!("Time {}: half-sum {}", now, scaled);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx411<'info> {
    #[account(mut)]
    pub a: AccountInfo<'info>,
    #[account(mut)]
    pub b: AccountInfo<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Distinct accounts required")]
    Same,
}
