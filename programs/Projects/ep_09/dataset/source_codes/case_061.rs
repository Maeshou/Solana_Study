use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_061 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_061(ctx: Context<CloseStruct061>) -> ProgramResult {
        let mut acc_061_lam = ctx.accounts.acc_061.to_account_info().lamports.borrow_mut();
        let mut dest_061_lam = ctx.accounts.dest_061.to_account_info().lamports.borrow_mut();
        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_061.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_061.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_061.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_061.to_account_info().lamports.borrow_mut() = 0;
    

        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_061_lam, *dest_061_lam);
        let new = *acc_061_lam + *dest_061_lam;
        *dest_061_lam = new;
    

        // Snippet: helper function abstraction
        fn transfer(src: &mut u64, dst: &mut u64) {
            let tmp = **src;
            **src = 0;
            **dst = dst.checked_add(tmp).unwrap();
        }
        transfer(&mut ctx.accounts.acc_061.to_account_info().lamports.borrow_mut(),
                 &mut ctx.accounts.dest_061.to_account_info().lamports.borrow_mut());
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct061<'info> {
    #[account(mut)]
    pub acc_061: AccountInfo<'info>,
    #[account(mut)]
    pub dest_061: AccountInfo<'info>,
}
