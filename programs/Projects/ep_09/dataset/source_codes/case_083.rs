use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_083 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_083(ctx: Context<CloseStruct083>) -> ProgramResult {
        let mut acc_083_lam = ctx.accounts.acc_083.to_account_info().lamports.borrow_mut();
        let mut dest_083_lam = ctx.accounts.dest_083.to_account_info().lamports.borrow_mut();
        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_083.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_083.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_083.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_083.to_account_info().lamports.borrow_mut() = 0;
    

        // Snippet: reversed zero then add
        *acc_083_lam = 0;
        let updated = (*dest_083_lam)
            .checked_add(*acc_083_lam)
            .unwrap();
        *dest_083_lam = updated;
    

        // Snippet: checked_add with unwrap
        *dest_083_lam = dest_083_lam.checked_add(*acc_083_lam).unwrap();
        *acc_083_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct083<'info> {
    #[account(mut)]
    pub acc_083: AccountInfo<'info>,
    #[account(mut)]
    pub dest_083: AccountInfo<'info>,
}
