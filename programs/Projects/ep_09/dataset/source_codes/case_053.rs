use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_053 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_053(ctx: Context<CloseStruct053>) -> ProgramResult {
        let mut acc_053_lam = ctx.accounts.acc_053.to_account_info().lamports.borrow_mut();
        let mut dest_053_lam = ctx.accounts.dest_053.to_account_info().lamports.borrow_mut();
        // Snippet: reversed zero then add
        *acc_053_lam = 0;
        let updated = (*dest_053_lam)
            .checked_add(*acc_053_lam)
            .unwrap();
        *dest_053_lam = updated;
    

        // Snippet: helper function abstraction
        fn transfer(src: &mut u64, dst: &mut u64) {
            let tmp = **src;
            **src = 0;
            **dst = dst.checked_add(tmp).unwrap();
        }
        transfer(&mut ctx.accounts.acc_053.to_account_info().lamports.borrow_mut(),
                 &mut ctx.accounts.dest_053.to_account_info().lamports.borrow_mut());
    

        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_053.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_053.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_053.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_053.to_account_info().lamports.borrow_mut() = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct053<'info> {
    #[account(mut)]
    pub acc_053: AccountInfo<'info>,
    #[account(mut)]
    pub dest_053: AccountInfo<'info>,
}
