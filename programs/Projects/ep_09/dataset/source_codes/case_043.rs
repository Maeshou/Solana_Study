use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_043 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_043(ctx: Context<CloseStruct043>) -> ProgramResult {
        let mut acc_043_lam = ctx.accounts.acc_043.to_account_info().lamports.borrow_mut();
        let mut dest_043_lam = ctx.accounts.dest_043.to_account_info().lamports.borrow_mut();
        // Snippet: plain add and zero
        let sum = *acc_043_lam + *dest_043_lam;
        *dest_043_lam = sum;
        *acc_043_lam = 0;
    

        // Snippet: nested unwrap pattern
        *dest_043_lam = dest_043_lam.checked_add(acc_043_lam.checked_add(0).unwrap()).unwrap();
        *acc_043_lam = acc_043_lam.checked_sub(acc_043_lam).unwrap();
    

        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_043_lam, *dest_043_lam);
        let new = *acc_043_lam + *dest_043_lam;
        *dest_043_lam = new;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct043<'info> {
    #[account(mut)]
    pub acc_043: AccountInfo<'info>,
    #[account(mut)]
    pub dest_043: AccountInfo<'info>,
}
