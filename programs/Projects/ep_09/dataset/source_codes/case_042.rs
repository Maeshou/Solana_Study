use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_042 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_042(ctx: Context<CloseStruct042>) -> ProgramResult {
        let mut acc_042_lam = ctx.accounts.acc_042.to_account_info().lamports.borrow_mut();
        let mut dest_042_lam = ctx.accounts.dest_042.to_account_info().lamports.borrow_mut();
        // Snippet: checked_add with unwrap
        *dest_042_lam = dest_042_lam.checked_add(*acc_042_lam).unwrap();
        *acc_042_lam = 0;
    

        // Snippet: plain add and zero
        let sum = *acc_042_lam + *dest_042_lam;
        *dest_042_lam = sum;
        *acc_042_lam = 0;
    

        // Snippet: nested unwrap pattern
        *dest_042_lam = dest_042_lam.checked_add(acc_042_lam.checked_add(0).unwrap()).unwrap();
        *acc_042_lam = acc_042_lam.checked_sub(acc_042_lam).unwrap();
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct042<'info> {
    #[account(mut)]
    pub acc_042: AccountInfo<'info>,
    #[account(mut)]
    pub dest_042: AccountInfo<'info>,
}
