use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_012 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_012(ctx: Context<CloseStruct012>) -> ProgramResult {
        let mut acc_012_lam = ctx.accounts.acc_012.to_account_info().lamports.borrow_mut();
        let mut dest_012_lam = ctx.accounts.dest_012.to_account_info().lamports.borrow_mut();
        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_012.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_012.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_012.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_012.to_account_info().lamports.borrow_mut() = 0;
    

        // Snippet: nested unwrap pattern
        *dest_012_lam = dest_012_lam.checked_add(acc_012_lam.checked_add(0).unwrap()).unwrap();
        *acc_012_lam = acc_012_lam.checked_sub(acc_012_lam).unwrap();
    

        // Snippet: alias reference
        let dst_ref: &mut u64 = &mut *dest_012_lam;
        let src_val = *acc_012_lam;
        *dst_ref = dst_ref.checked_add(src_val).unwrap();
        *acc_012_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct012<'info> {
    #[account(mut)]
    pub acc_012: AccountInfo<'info>,
    #[account(mut)]
    pub dest_012: AccountInfo<'info>,
}
