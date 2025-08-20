use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_014 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_014(ctx: Context<CloseStruct014>) -> ProgramResult {
        let mut acc_014_lam = ctx.accounts.acc_014.to_account_info().lamports.borrow_mut();
        let mut dest_014_lam = ctx.accounts.dest_014.to_account_info().lamports.borrow_mut();
        // Snippet: checked_add with unwrap
        *dest_014_lam = dest_014_lam.checked_add(*acc_014_lam).unwrap();
        *acc_014_lam = 0;
    

        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_014_lam = dest_014_lam.checked_add(*acc_014_lam).unwrap();
        *acc_014_lam = 0;
    

        // Snippet: helper function abstraction
        fn transfer(src: &mut u64, dst: &mut u64) {
            let tmp = **src;
            **src = 0;
            **dst = dst.checked_add(tmp).unwrap();
        }
        transfer(&mut ctx.accounts.acc_014.to_account_info().lamports.borrow_mut(),
                 &mut ctx.accounts.dest_014.to_account_info().lamports.borrow_mut());
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct014<'info> {
    #[account(mut)]
    pub acc_014: AccountInfo<'info>,
    #[account(mut)]
    pub dest_014: AccountInfo<'info>,
}
