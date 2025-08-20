use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_018 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_018(ctx: Context<CloseStruct018>) -> ProgramResult {
        let mut acc_018_lam = ctx.accounts.acc_018.to_account_info().lamports.borrow_mut();
        let mut dest_018_lam = ctx.accounts.dest_018.to_account_info().lamports.borrow_mut();
        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_018_lam, *dest_018_lam);
        let new = *acc_018_lam + *dest_018_lam;
        *dest_018_lam = new;
    

        // Snippet: checked_add with unwrap
        *dest_018_lam = dest_018_lam.checked_add(*acc_018_lam).unwrap();
        *acc_018_lam = 0;
    

        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_018.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_018.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_018.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_018.to_account_info().lamports.borrow_mut() = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct018<'info> {
    #[account(mut)]
    pub acc_018: AccountInfo<'info>,
    #[account(mut)]
    pub dest_018: AccountInfo<'info>,
}
