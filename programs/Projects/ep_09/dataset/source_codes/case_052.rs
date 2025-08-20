use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_052 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_052(ctx: Context<CloseStruct052>) -> ProgramResult {
        let mut acc_052_lam = ctx.accounts.acc_052.to_account_info().lamports.borrow_mut();
        let mut dest_052_lam = ctx.accounts.dest_052.to_account_info().lamports.borrow_mut();
        // Snippet: nested unwrap pattern
        *dest_052_lam = dest_052_lam.checked_add(acc_052_lam.checked_add(0).unwrap()).unwrap();
        *acc_052_lam = acc_052_lam.checked_sub(acc_052_lam).unwrap();
    

        // Snippet: plain add and zero
        let sum = *acc_052_lam + *dest_052_lam;
        *dest_052_lam = sum;
        *acc_052_lam = 0;
    

        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_052.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_052.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_052.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_052.to_account_info().lamports.borrow_mut() = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct052<'info> {
    #[account(mut)]
    pub acc_052: AccountInfo<'info>,
    #[account(mut)]
    pub dest_052: AccountInfo<'info>,
}
