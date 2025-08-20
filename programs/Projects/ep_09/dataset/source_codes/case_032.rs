use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_032 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_032(ctx: Context<CloseStruct032>) -> ProgramResult {
        let mut acc_032_lam = ctx.accounts.acc_032.to_account_info().lamports.borrow_mut();
        let mut dest_032_lam = ctx.accounts.dest_032.to_account_info().lamports.borrow_mut();
        // Snippet: checked_add with unwrap
        *dest_032_lam = dest_032_lam.checked_add(*acc_032_lam).unwrap();
        *acc_032_lam = 0;
    

        // Snippet: helper function abstraction
        fn transfer(src: &mut u64, dst: &mut u64) {
            let tmp = **src;
            **src = 0;
            **dst = dst.checked_add(tmp).unwrap();
        }
        transfer(&mut ctx.accounts.acc_032.to_account_info().lamports.borrow_mut(),
                 &mut ctx.accounts.dest_032.to_account_info().lamports.borrow_mut());
    

        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_032_lam, *dest_032_lam);
        let new = *acc_032_lam + *dest_032_lam;
        *dest_032_lam = new;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct032<'info> {
    #[account(mut)]
    pub acc_032: AccountInfo<'info>,
    #[account(mut)]
    pub dest_032: AccountInfo<'info>,
}
