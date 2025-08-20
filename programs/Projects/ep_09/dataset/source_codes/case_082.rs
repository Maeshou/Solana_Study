use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_082 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_082(ctx: Context<CloseStruct082>) -> ProgramResult {
        let mut acc_082_lam = ctx.accounts.acc_082.to_account_info().lamports.borrow_mut();
        let mut dest_082_lam = ctx.accounts.dest_082.to_account_info().lamports.borrow_mut();
        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_082_lam, *dest_082_lam);
        let new = *acc_082_lam + *dest_082_lam;
        *dest_082_lam = new;
    

        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_082.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_082.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_082.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_082.to_account_info().lamports.borrow_mut() = 0;
    

        // Snippet: nested unwrap pattern
        *dest_082_lam = dest_082_lam.checked_add(acc_082_lam.checked_add(0).unwrap()).unwrap();
        *acc_082_lam = acc_082_lam.checked_sub(acc_082_lam).unwrap();
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct082<'info> {
    #[account(mut)]
    pub acc_082: AccountInfo<'info>,
    #[account(mut)]
    pub dest_082: AccountInfo<'info>,
}
