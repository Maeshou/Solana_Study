use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_033 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_033(ctx: Context<CloseStruct033>) -> ProgramResult {
        let mut acc_033_lam = ctx.accounts.acc_033.to_account_info().lamports.borrow_mut();
        let mut dest_033_lam = ctx.accounts.dest_033.to_account_info().lamports.borrow_mut();
        // Snippet: reversed zero then add
        *acc_033_lam = 0;
        let updated = (*dest_033_lam)
            .checked_add(*acc_033_lam)
            .unwrap();
        *dest_033_lam = updated;
    

        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_033_lam, *dest_033_lam);
        let new = *acc_033_lam + *dest_033_lam;
        *dest_033_lam = new;
    

        // Snippet: helper function abstraction
        fn transfer(src: &mut u64, dst: &mut u64) {
            let tmp = **src;
            **src = 0;
            **dst = dst.checked_add(tmp).unwrap();
        }
        transfer(&mut ctx.accounts.acc_033.to_account_info().lamports.borrow_mut(),
                 &mut ctx.accounts.dest_033.to_account_info().lamports.borrow_mut());
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct033<'info> {
    #[account(mut)]
    pub acc_033: AccountInfo<'info>,
    #[account(mut)]
    pub dest_033: AccountInfo<'info>,
}
