use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_070 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_070(ctx: Context<CloseStruct070>) -> ProgramResult {
        let mut acc_070_lam = ctx.accounts.acc_070.to_account_info().lamports.borrow_mut();
        let mut dest_070_lam = ctx.accounts.dest_070.to_account_info().lamports.borrow_mut();
        // Snippet: checked_add with unwrap
        *dest_070_lam = dest_070_lam.checked_add(*acc_070_lam).unwrap();
        *acc_070_lam = 0;
    

        // Snippet: nested unwrap pattern
        *dest_070_lam = dest_070_lam.checked_add(acc_070_lam.checked_add(0).unwrap()).unwrap();
        *acc_070_lam = acc_070_lam.checked_sub(acc_070_lam).unwrap();
    

        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_070.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_070.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_070.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_070.to_account_info().lamports.borrow_mut() = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct070<'info> {
    #[account(mut)]
    pub acc_070: AccountInfo<'info>,
    #[account(mut)]
    pub dest_070: AccountInfo<'info>,
}
