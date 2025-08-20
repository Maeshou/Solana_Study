use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_051 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_051(ctx: Context<CloseStruct051>) -> ProgramResult {
        let mut acc_051_lam = ctx.accounts.acc_051.to_account_info().lamports.borrow_mut();
        let mut dest_051_lam = ctx.accounts.dest_051.to_account_info().lamports.borrow_mut();
        // Snippet: helper function abstraction
        fn transfer(src: &mut u64, dst: &mut u64) {
            let tmp = **src;
            **src = 0;
            **dst = dst.checked_add(tmp).unwrap();
        }
        transfer(&mut ctx.accounts.acc_051.to_account_info().lamports.borrow_mut(),
                 &mut ctx.accounts.dest_051.to_account_info().lamports.borrow_mut());
    

        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_051_lam = dest_051_lam.checked_add(*acc_051_lam).unwrap();
        *acc_051_lam = 0;
    

        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_051_lam, *dest_051_lam);
        let new = *acc_051_lam + *dest_051_lam;
        *dest_051_lam = new;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct051<'info> {
    #[account(mut)]
    pub acc_051: AccountInfo<'info>,
    #[account(mut)]
    pub dest_051: AccountInfo<'info>,
}
