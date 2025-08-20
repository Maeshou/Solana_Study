use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_054 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_054(ctx: Context<CloseStruct054>) -> ProgramResult {
        let mut acc_054_lam = ctx.accounts.acc_054.to_account_info().lamports.borrow_mut();
        let mut dest_054_lam = ctx.accounts.dest_054.to_account_info().lamports.borrow_mut();
        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_054_lam, *dest_054_lam);
        let new = *acc_054_lam + *dest_054_lam;
        *dest_054_lam = new;
    

        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_054.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_054.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_054.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_054.to_account_info().lamports.borrow_mut() = 0;
    

        // Snippet: nested unwrap pattern
        *dest_054_lam = dest_054_lam.checked_add(acc_054_lam.checked_add(0).unwrap()).unwrap();
        *acc_054_lam = acc_054_lam.checked_sub(acc_054_lam).unwrap();
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct054<'info> {
    #[account(mut)]
    pub acc_054: AccountInfo<'info>,
    #[account(mut)]
    pub dest_054: AccountInfo<'info>,
}
