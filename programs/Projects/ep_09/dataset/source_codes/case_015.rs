use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_015 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_015(ctx: Context<CloseStruct015>) -> ProgramResult {
        let mut acc_015_lam = ctx.accounts.acc_015.to_account_info().lamports.borrow_mut();
        let mut dest_015_lam = ctx.accounts.dest_015.to_account_info().lamports.borrow_mut();
        // Snippet: helper function abstraction
        fn transfer(src: &mut u64, dst: &mut u64) {
            let tmp = **src;
            **src = 0;
            **dst = dst.checked_add(tmp).unwrap();
        }
        transfer(&mut ctx.accounts.acc_015.to_account_info().lamports.borrow_mut(),
                 &mut ctx.accounts.dest_015.to_account_info().lamports.borrow_mut());
    

        // Snippet: plain add and zero
        let sum = *acc_015_lam + *dest_015_lam;
        *dest_015_lam = sum;
        *acc_015_lam = 0;
    

        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_015_lam = dest_015_lam.checked_add(*acc_015_lam).unwrap();
        *acc_015_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct015<'info> {
    #[account(mut)]
    pub acc_015: AccountInfo<'info>,
    #[account(mut)]
    pub dest_015: AccountInfo<'info>,
}
