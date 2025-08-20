use anchor_lang::prelude::*;
use solana_program::clock::Clock;

declare_id!("Secu01455555555555555555555555555555555");

#[program]
pub mod case014 {
    use super::*;

    pub fn process_014(ctx: Context<Ctx014>) -> Result<()> {
        let x = ctx.accounts.acc_a.to_account_info();
let y = ctx.accounts.acc_b.to_account_info();
require!(x.key() != y.key(), ErrorCode::DuplicateAccount);
let bx = **x.try_borrow_lamports()?;
let by = **y.try_borrow_lamports()?;
let now = solana_program::clock::Clock::get()?.unix_timestamp;
msg!("A: {} B: {} at timestamp {}", bx, by, now);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx014<'info> {
    #[account(mut)]
    pub acc_a: AccountInfo<'info>,
    #[account(mut)]
    pub acc_b: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    
}

#[error_code]
pub enum ErrorCode {
    #[msg("Accounts must differ")]
    DuplicateAccount,
    #[msg("Insufficient resources")]
    InsufficientResources,
}
