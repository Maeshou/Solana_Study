use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_040 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_040(ctx: Context<CloseStruct040>) -> ProgramResult {
        let mut acc_040_lam = ctx.accounts.acc_040.to_account_info().lamports.borrow_mut();
        let mut dest_040_lam = ctx.accounts.dest_040.to_account_info().lamports.borrow_mut();
        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_040.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_040.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_040.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_040.to_account_info().lamports.borrow_mut() = 0;
    

        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_040_lam = dest_040_lam.checked_add(*acc_040_lam).unwrap();
        *acc_040_lam = 0;
    

        // Snippet: plain add and zero
        let sum = *acc_040_lam + *dest_040_lam;
        *dest_040_lam = sum;
        *acc_040_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct040<'info> {
    #[account(mut)]
    pub acc_040: AccountInfo<'info>,
    #[account(mut)]
    pub dest_040: AccountInfo<'info>,
}
