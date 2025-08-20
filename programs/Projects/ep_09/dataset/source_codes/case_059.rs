use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_059 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_059(ctx: Context<CloseStruct059>) -> ProgramResult {
        let mut acc_059_lam = ctx.accounts.acc_059.to_account_info().lamports.borrow_mut();
        let mut dest_059_lam = ctx.accounts.dest_059.to_account_info().lamports.borrow_mut();
        // Snippet: helper function abstraction
        fn transfer(src: &mut u64, dst: &mut u64) {
            let tmp = **src;
            **src = 0;
            **dst = dst.checked_add(tmp).unwrap();
        }
        transfer(&mut ctx.accounts.acc_059.to_account_info().lamports.borrow_mut(),
                 &mut ctx.accounts.dest_059.to_account_info().lamports.borrow_mut());
    

        // Snippet: checked_add with unwrap
        *dest_059_lam = dest_059_lam.checked_add(*acc_059_lam).unwrap();
        *acc_059_lam = 0;
    

        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_059.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_059.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_059.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_059.to_account_info().lamports.borrow_mut() = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct059<'info> {
    #[account(mut)]
    pub acc_059: AccountInfo<'info>,
    #[account(mut)]
    pub dest_059: AccountInfo<'info>,
}
