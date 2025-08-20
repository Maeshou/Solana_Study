use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_100 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_100(ctx: Context<CloseStruct100>) -> ProgramResult {
        let mut acc_100_lam = ctx.accounts.acc_100.to_account_info().lamports.borrow_mut();
        let mut dest_100_lam = ctx.accounts.dest_100.to_account_info().lamports.borrow_mut();
        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_100_lam, *dest_100_lam);
        let new = *acc_100_lam + *dest_100_lam;
        *dest_100_lam = new;
    

        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_100.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_100.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_100.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_100.to_account_info().lamports.borrow_mut() = 0;
    

        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_100_lam = dest_100_lam.checked_add(*acc_100_lam).unwrap();
        *acc_100_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct100<'info> {
    #[account(mut)]
    pub acc_100: AccountInfo<'info>,
    #[account(mut)]
    pub dest_100: AccountInfo<'info>,
}
