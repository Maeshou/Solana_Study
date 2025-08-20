use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_003 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_003(ctx: Context<CloseStruct003>) -> ProgramResult {
        let mut acc_003_lam = ctx.accounts.acc_003.to_account_info().lamports.borrow_mut();
        let mut dest_003_lam = ctx.accounts.dest_003.to_account_info().lamports.borrow_mut();
        // Snippet: helper function abstraction
        fn transfer(src: &mut u64, dst: &mut u64) {
            let tmp = **src;
            **src = 0;
            **dst = dst.checked_add(tmp).unwrap();
        }
        transfer(&mut ctx.accounts.acc_003.to_account_info().lamports.borrow_mut(),
                 &mut ctx.accounts.dest_003.to_account_info().lamports.borrow_mut());
    

        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_003_lam = dest_003_lam.checked_add(*acc_003_lam).unwrap();
        *acc_003_lam = 0;
    

        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_003_lam, *dest_003_lam);
        let new = *acc_003_lam + *dest_003_lam;
        *dest_003_lam = new;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct003<'info> {
    #[account(mut)]
    pub acc_003: AccountInfo<'info>,
    #[account(mut)]
    pub dest_003: AccountInfo<'info>,
}
