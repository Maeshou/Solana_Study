use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_076 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_076(ctx: Context<CloseStruct076>) -> ProgramResult {
        let mut acc_076_lam = ctx.accounts.acc_076.to_account_info().lamports.borrow_mut();
        let mut dest_076_lam = ctx.accounts.dest_076.to_account_info().lamports.borrow_mut();
        // Snippet: alias reference
        let dst_ref: &mut u64 = &mut *dest_076_lam;
        let src_val = *acc_076_lam;
        *dst_ref = dst_ref.checked_add(src_val).unwrap();
        *acc_076_lam = 0;
    

        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_076.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_076.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_076.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_076.to_account_info().lamports.borrow_mut() = 0;
    

        // Snippet: unwrap_or_default fallback
        let new_dst = dest_076_lam.checked_add(*acc_076_lam).unwrap_or_default();
        *dest_076_lam = new_dst;
        *acc_076_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct076<'info> {
    #[account(mut)]
    pub acc_076: AccountInfo<'info>,
    #[account(mut)]
    pub dest_076: AccountInfo<'info>,
}
