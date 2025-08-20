use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_028 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_028(ctx: Context<CloseStruct028>) -> ProgramResult {
        let mut acc_028_lam = ctx.accounts.acc_028.to_account_info().lamports.borrow_mut();
        let mut dest_028_lam = ctx.accounts.dest_028.to_account_info().lamports.borrow_mut();
        // Snippet: checked_add with unwrap
        *dest_028_lam = dest_028_lam.checked_add(*acc_028_lam).unwrap();
        *acc_028_lam = 0;
    

        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_028_lam, *dest_028_lam);
        let new = *acc_028_lam + *dest_028_lam;
        *dest_028_lam = new;
    

        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_028_lam = dest_028_lam.checked_add(*acc_028_lam).unwrap();
        *acc_028_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct028<'info> {
    #[account(mut)]
    pub acc_028: AccountInfo<'info>,
    #[account(mut)]
    pub dest_028: AccountInfo<'info>,
}
