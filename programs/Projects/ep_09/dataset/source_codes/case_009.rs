use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_009 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_009(ctx: Context<CloseStruct009>) -> ProgramResult {
        let mut acc_009_lam = ctx.accounts.acc_009.to_account_info().lamports.borrow_mut();
        let mut dest_009_lam = ctx.accounts.dest_009.to_account_info().lamports.borrow_mut();
        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_009.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_009.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_009.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_009.to_account_info().lamports.borrow_mut() = 0;
    

        // Snippet: alias reference
        let dst_ref: &mut u64 = &mut *dest_009_lam;
        let src_val = *acc_009_lam;
        *dst_ref = dst_ref.checked_add(src_val).unwrap();
        *acc_009_lam = 0;
    

        // Snippet: nested unwrap pattern
        *dest_009_lam = dest_009_lam.checked_add(acc_009_lam.checked_add(0).unwrap()).unwrap();
        *acc_009_lam = acc_009_lam.checked_sub(acc_009_lam).unwrap();
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct009<'info> {
    #[account(mut)]
    pub acc_009: AccountInfo<'info>,
    #[account(mut)]
    pub dest_009: AccountInfo<'info>,
}
