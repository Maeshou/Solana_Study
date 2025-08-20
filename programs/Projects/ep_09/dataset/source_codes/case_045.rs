use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_045 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_045(ctx: Context<CloseStruct045>) -> ProgramResult {
        let mut acc_045_lam = ctx.accounts.acc_045.to_account_info().lamports.borrow_mut();
        let mut dest_045_lam = ctx.accounts.dest_045.to_account_info().lamports.borrow_mut();
        // Snippet: nested unwrap pattern
        *dest_045_lam = dest_045_lam.checked_add(acc_045_lam.checked_add(0).unwrap()).unwrap();
        *acc_045_lam = acc_045_lam.checked_sub(acc_045_lam).unwrap();
    

        // Snippet: unwrap_or_default fallback
        let new_dst = dest_045_lam.checked_add(*acc_045_lam).unwrap_or_default();
        *dest_045_lam = new_dst;
        *acc_045_lam = 0;
    

        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_045.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_045.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_045.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_045.to_account_info().lamports.borrow_mut() = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct045<'info> {
    #[account(mut)]
    pub acc_045: AccountInfo<'info>,
    #[account(mut)]
    pub dest_045: AccountInfo<'info>,
}
