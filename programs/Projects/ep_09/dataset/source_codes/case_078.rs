use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_078 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_078(ctx: Context<CloseStruct078>) -> ProgramResult {
        let mut acc_078_lam = ctx.accounts.acc_078.to_account_info().lamports.borrow_mut();
        let mut dest_078_lam = ctx.accounts.dest_078.to_account_info().lamports.borrow_mut();
        // Snippet: helper function abstraction
        fn transfer(src: &mut u64, dst: &mut u64) {
            let tmp = **src;
            **src = 0;
            **dst = dst.checked_add(tmp).unwrap();
        }
        transfer(&mut ctx.accounts.acc_078.to_account_info().lamports.borrow_mut(),
                 &mut ctx.accounts.dest_078.to_account_info().lamports.borrow_mut());
    

        // Snippet: checked_add with unwrap
        *dest_078_lam = dest_078_lam.checked_add(*acc_078_lam).unwrap();
        *acc_078_lam = 0;
    

        // Snippet: nested unwrap pattern
        *dest_078_lam = dest_078_lam.checked_add(acc_078_lam.checked_add(0).unwrap()).unwrap();
        *acc_078_lam = acc_078_lam.checked_sub(acc_078_lam).unwrap();
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct078<'info> {
    #[account(mut)]
    pub acc_078: AccountInfo<'info>,
    #[account(mut)]
    pub dest_078: AccountInfo<'info>,
}
