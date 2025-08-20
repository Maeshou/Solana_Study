use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_016 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_016(ctx: Context<CloseStruct016>) -> ProgramResult {
        let mut acc_016_lam = ctx.accounts.acc_016.to_account_info().lamports.borrow_mut();
        let mut dest_016_lam = ctx.accounts.dest_016.to_account_info().lamports.borrow_mut();
        // Snippet: alias reference
        let dst_ref: &mut u64 = &mut *dest_016_lam;
        let src_val = *acc_016_lam;
        *dst_ref = dst_ref.checked_add(src_val).unwrap();
        *acc_016_lam = 0;
    

        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_016.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_016.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_016.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_016.to_account_info().lamports.borrow_mut() = 0;
    

        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_016_lam = dest_016_lam.checked_add(*acc_016_lam).unwrap();
        *acc_016_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct016<'info> {
    #[account(mut)]
    pub acc_016: AccountInfo<'info>,
    #[account(mut)]
    pub dest_016: AccountInfo<'info>,
}
