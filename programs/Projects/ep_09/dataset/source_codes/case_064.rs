use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_064 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_064(ctx: Context<CloseStruct064>) -> ProgramResult {
        let mut acc_064_lam = ctx.accounts.acc_064.to_account_info().lamports.borrow_mut();
        let mut dest_064_lam = ctx.accounts.dest_064.to_account_info().lamports.borrow_mut();
        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_064.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_064.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_064.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_064.to_account_info().lamports.borrow_mut() = 0;
    

        // Snippet: checked_add with unwrap
        *dest_064_lam = dest_064_lam.checked_add(*acc_064_lam).unwrap();
        *acc_064_lam = 0;
    

        // Snippet: unwrap_or_default fallback
        let new_dst = dest_064_lam.checked_add(*acc_064_lam).unwrap_or_default();
        *dest_064_lam = new_dst;
        *acc_064_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct064<'info> {
    #[account(mut)]
    pub acc_064: AccountInfo<'info>,
    #[account(mut)]
    pub dest_064: AccountInfo<'info>,
}
