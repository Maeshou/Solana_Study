use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_084 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_084(ctx: Context<CloseStruct084>) -> ProgramResult {
        let mut acc_084_lam = ctx.accounts.acc_084.to_account_info().lamports.borrow_mut();
        let mut dest_084_lam = ctx.accounts.dest_084.to_account_info().lamports.borrow_mut();
        // Snippet: nested unwrap pattern
        *dest_084_lam = dest_084_lam.checked_add(acc_084_lam.checked_add(0).unwrap()).unwrap();
        *acc_084_lam = acc_084_lam.checked_sub(acc_084_lam).unwrap();
    

        // Snippet: plain add and zero
        let sum = *acc_084_lam + *dest_084_lam;
        *dest_084_lam = sum;
        *acc_084_lam = 0;
    

        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_084.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_084.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_084.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_084.to_account_info().lamports.borrow_mut() = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct084<'info> {
    #[account(mut)]
    pub acc_084: AccountInfo<'info>,
    #[account(mut)]
    pub dest_084: AccountInfo<'info>,
}
