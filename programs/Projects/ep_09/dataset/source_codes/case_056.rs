use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_056 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_056(ctx: Context<CloseStruct056>) -> ProgramResult {
        let mut acc_056_lam = ctx.accounts.acc_056.to_account_info().lamports.borrow_mut();
        let mut dest_056_lam = ctx.accounts.dest_056.to_account_info().lamports.borrow_mut();
        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_056_lam = dest_056_lam.checked_add(*acc_056_lam).unwrap();
        *acc_056_lam = 0;
    

        // Snippet: plain add and zero
        let sum = *acc_056_lam + *dest_056_lam;
        *dest_056_lam = sum;
        *acc_056_lam = 0;
    

        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_056.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_056.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_056.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_056.to_account_info().lamports.borrow_mut() = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct056<'info> {
    #[account(mut)]
    pub acc_056: AccountInfo<'info>,
    #[account(mut)]
    pub dest_056: AccountInfo<'info>,
}
