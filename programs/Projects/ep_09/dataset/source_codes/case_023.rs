use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_023 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_023(ctx: Context<CloseStruct023>) -> ProgramResult {
        let mut acc_023_lam = ctx.accounts.acc_023.to_account_info().lamports.borrow_mut();
        let mut dest_023_lam = ctx.accounts.dest_023.to_account_info().lamports.borrow_mut();
        // Snippet: nested unwrap pattern
        *dest_023_lam = dest_023_lam.checked_add(acc_023_lam.checked_add(0).unwrap()).unwrap();
        *acc_023_lam = acc_023_lam.checked_sub(acc_023_lam).unwrap();
    

        // Snippet: checked_add with unwrap
        *dest_023_lam = dest_023_lam.checked_add(*acc_023_lam).unwrap();
        *acc_023_lam = 0;
    

        // Snippet: plain add and zero
        let sum = *acc_023_lam + *dest_023_lam;
        *dest_023_lam = sum;
        *acc_023_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct023<'info> {
    #[account(mut)]
    pub acc_023: AccountInfo<'info>,
    #[account(mut)]
    pub dest_023: AccountInfo<'info>,
}
