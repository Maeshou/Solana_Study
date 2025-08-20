use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_093 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_093(ctx: Context<CloseStruct093>) -> ProgramResult {
        let mut acc_093_lam = ctx.accounts.acc_093.to_account_info().lamports.borrow_mut();
        let mut dest_093_lam = ctx.accounts.dest_093.to_account_info().lamports.borrow_mut();
        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_093.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_093.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_093.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_093.to_account_info().lamports.borrow_mut() = 0;
    

        // Snippet: alias reference
        let dst_ref: &mut u64 = &mut *dest_093_lam;
        let src_val = *acc_093_lam;
        *dst_ref = dst_ref.checked_add(src_val).unwrap();
        *acc_093_lam = 0;
    

        // Snippet: checked_add with unwrap
        *dest_093_lam = dest_093_lam.checked_add(*acc_093_lam).unwrap();
        *acc_093_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct093<'info> {
    #[account(mut)]
    pub acc_093: AccountInfo<'info>,
    #[account(mut)]
    pub dest_093: AccountInfo<'info>,
}
