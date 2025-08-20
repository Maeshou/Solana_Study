use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_031 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_031(ctx: Context<CloseStruct031>) -> ProgramResult {
        let mut acc_031_lam = ctx.accounts.acc_031.to_account_info().lamports.borrow_mut();
        let mut dest_031_lam = ctx.accounts.dest_031.to_account_info().lamports.borrow_mut();
        // Snippet: plain add and zero
        let sum = *acc_031_lam + *dest_031_lam;
        *dest_031_lam = sum;
        *acc_031_lam = 0;
    

        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_031.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_031.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_031.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_031.to_account_info().lamports.borrow_mut() = 0;
    

        // Snippet: checked_add with unwrap
        *dest_031_lam = dest_031_lam.checked_add(*acc_031_lam).unwrap();
        *acc_031_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct031<'info> {
    #[account(mut)]
    pub acc_031: AccountInfo<'info>,
    #[account(mut)]
    pub dest_031: AccountInfo<'info>,
}
