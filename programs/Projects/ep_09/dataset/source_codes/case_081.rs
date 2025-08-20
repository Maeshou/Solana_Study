use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_081 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_081(ctx: Context<CloseStruct081>) -> ProgramResult {
        let mut acc_081_lam = ctx.accounts.acc_081.to_account_info().lamports.borrow_mut();
        let mut dest_081_lam = ctx.accounts.dest_081.to_account_info().lamports.borrow_mut();
        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_081_lam = dest_081_lam.checked_add(*acc_081_lam).unwrap();
        *acc_081_lam = 0;
    

        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_081.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_081.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_081.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_081.to_account_info().lamports.borrow_mut() = 0;
    

        // Snippet: alias reference
        let dst_ref: &mut u64 = &mut *dest_081_lam;
        let src_val = *acc_081_lam;
        *dst_ref = dst_ref.checked_add(src_val).unwrap();
        *acc_081_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct081<'info> {
    #[account(mut)]
    pub acc_081: AccountInfo<'info>,
    #[account(mut)]
    pub dest_081: AccountInfo<'info>,
}
