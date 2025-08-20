use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_050 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_050(ctx: Context<CloseStruct050>) -> ProgramResult {
        let mut acc_050_lam = ctx.accounts.acc_050.to_account_info().lamports.borrow_mut();
        let mut dest_050_lam = ctx.accounts.dest_050.to_account_info().lamports.borrow_mut();
        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_050.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_050.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_050.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_050.to_account_info().lamports.borrow_mut() = 0;
    

        // Snippet: nested unwrap pattern
        *dest_050_lam = dest_050_lam.checked_add(acc_050_lam.checked_add(0).unwrap()).unwrap();
        *acc_050_lam = acc_050_lam.checked_sub(acc_050_lam).unwrap();
    

        // Snippet: alias reference
        let dst_ref: &mut u64 = &mut *dest_050_lam;
        let src_val = *acc_050_lam;
        *dst_ref = dst_ref.checked_add(src_val).unwrap();
        *acc_050_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct050<'info> {
    #[account(mut)]
    pub acc_050: AccountInfo<'info>,
    #[account(mut)]
    pub dest_050: AccountInfo<'info>,
}
