use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_022 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_022(ctx: Context<CloseStruct022>) -> ProgramResult {
        let mut acc_022_lam = ctx.accounts.acc_022.to_account_info().lamports.borrow_mut();
        let mut dest_022_lam = ctx.accounts.dest_022.to_account_info().lamports.borrow_mut();
        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_022_lam, *dest_022_lam);
        let new = *acc_022_lam + *dest_022_lam;
        *dest_022_lam = new;
    

        // Snippet: helper function abstraction
        fn transfer(src: &mut u64, dst: &mut u64) {
            let tmp = **src;
            **src = 0;
            **dst = dst.checked_add(tmp).unwrap();
        }
        transfer(&mut ctx.accounts.acc_022.to_account_info().lamports.borrow_mut(),
                 &mut ctx.accounts.dest_022.to_account_info().lamports.borrow_mut());
    

        // Snippet: plain add and zero
        let sum = *acc_022_lam + *dest_022_lam;
        *dest_022_lam = sum;
        *acc_022_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct022<'info> {
    #[account(mut)]
    pub acc_022: AccountInfo<'info>,
    #[account(mut)]
    pub dest_022: AccountInfo<'info>,
}
