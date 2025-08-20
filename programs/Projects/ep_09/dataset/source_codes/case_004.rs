use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_004 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_004(ctx: Context<CloseStruct004>) -> ProgramResult {
        let mut acc_004_lam = ctx.accounts.acc_004.to_account_info().lamports.borrow_mut();
        let mut dest_004_lam = ctx.accounts.dest_004.to_account_info().lamports.borrow_mut();
        // Snippet: helper function abstraction
        fn transfer(src: &mut u64, dst: &mut u64) {
            let tmp = **src;
            **src = 0;
            **dst = dst.checked_add(tmp).unwrap();
        }
        transfer(&mut ctx.accounts.acc_004.to_account_info().lamports.borrow_mut(),
                 &mut ctx.accounts.dest_004.to_account_info().lamports.borrow_mut());
    

        // Snippet: nested unwrap pattern
        *dest_004_lam = dest_004_lam.checked_add(acc_004_lam.checked_add(0).unwrap()).unwrap();
        *acc_004_lam = acc_004_lam.checked_sub(acc_004_lam).unwrap();
    

        // Snippet: checked_add with unwrap
        *dest_004_lam = dest_004_lam.checked_add(*acc_004_lam).unwrap();
        *acc_004_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct004<'info> {
    #[account(mut)]
    pub acc_004: AccountInfo<'info>,
    #[account(mut)]
    pub dest_004: AccountInfo<'info>,
}
