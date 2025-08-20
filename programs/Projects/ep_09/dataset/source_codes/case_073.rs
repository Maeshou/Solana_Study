use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_073 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_073(ctx: Context<CloseStruct073>) -> ProgramResult {
        let mut acc_073_lam = ctx.accounts.acc_073.to_account_info().lamports.borrow_mut();
        let mut dest_073_lam = ctx.accounts.dest_073.to_account_info().lamports.borrow_mut();
        // Snippet: nested unwrap pattern
        *dest_073_lam = dest_073_lam.checked_add(acc_073_lam.checked_add(0).unwrap()).unwrap();
        *acc_073_lam = acc_073_lam.checked_sub(acc_073_lam).unwrap();
    

        // Snippet: checked_add with unwrap
        *dest_073_lam = dest_073_lam.checked_add(*acc_073_lam).unwrap();
        *acc_073_lam = 0;
    

        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_073.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_073.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_073.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_073.to_account_info().lamports.borrow_mut() = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct073<'info> {
    #[account(mut)]
    pub acc_073: AccountInfo<'info>,
    #[account(mut)]
    pub dest_073: AccountInfo<'info>,
}
