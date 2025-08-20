use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_079 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_079(ctx: Context<CloseStruct079>) -> ProgramResult {
        let mut acc_079_lam = ctx.accounts.acc_079.to_account_info().lamports.borrow_mut();
        let mut dest_079_lam = ctx.accounts.dest_079.to_account_info().lamports.borrow_mut();
        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_079_lam, *dest_079_lam);
        let new = *acc_079_lam + *dest_079_lam;
        *dest_079_lam = new;
    

        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_079.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_079.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_079.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_079.to_account_info().lamports.borrow_mut() = 0;
    

        // Snippet: alias reference
        let dst_ref: &mut u64 = &mut *dest_079_lam;
        let src_val = *acc_079_lam;
        *dst_ref = dst_ref.checked_add(src_val).unwrap();
        *acc_079_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct079<'info> {
    #[account(mut)]
    pub acc_079: AccountInfo<'info>,
    #[account(mut)]
    pub dest_079: AccountInfo<'info>,
}
