use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_039 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_039(ctx: Context<CloseStruct039>) -> ProgramResult {
        let mut acc_039_lam = ctx.accounts.acc_039.to_account_info().lamports.borrow_mut();
        let mut dest_039_lam = ctx.accounts.dest_039.to_account_info().lamports.borrow_mut();
        // Snippet: helper function abstraction
        fn transfer(src: &mut u64, dst: &mut u64) {
            let tmp = **src;
            **src = 0;
            **dst = dst.checked_add(tmp).unwrap();
        }
        transfer(&mut ctx.accounts.acc_039.to_account_info().lamports.borrow_mut(),
                 &mut ctx.accounts.dest_039.to_account_info().lamports.borrow_mut());
    

        // Snippet: nested unwrap pattern
        *dest_039_lam = dest_039_lam.checked_add(acc_039_lam.checked_add(0).unwrap()).unwrap();
        *acc_039_lam = acc_039_lam.checked_sub(acc_039_lam).unwrap();
    

        // Snippet: unwrap_or_default fallback
        let new_dst = dest_039_lam.checked_add(*acc_039_lam).unwrap_or_default();
        *dest_039_lam = new_dst;
        *acc_039_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct039<'info> {
    #[account(mut)]
    pub acc_039: AccountInfo<'info>,
    #[account(mut)]
    pub dest_039: AccountInfo<'info>,
}
