use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_058 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_058(ctx: Context<CloseStruct058>) -> ProgramResult {
        let mut acc_058_lam = ctx.accounts.acc_058.to_account_info().lamports.borrow_mut();
        let mut dest_058_lam = ctx.accounts.dest_058.to_account_info().lamports.borrow_mut();
        // Snippet: reversed zero then add
        *acc_058_lam = 0;
        let updated = (*dest_058_lam)
            .checked_add(*acc_058_lam)
            .unwrap();
        *dest_058_lam = updated;
    

        // Snippet: helper function abstraction
        fn transfer(src: &mut u64, dst: &mut u64) {
            let tmp = **src;
            **src = 0;
            **dst = dst.checked_add(tmp).unwrap();
        }
        transfer(&mut ctx.accounts.acc_058.to_account_info().lamports.borrow_mut(),
                 &mut ctx.accounts.dest_058.to_account_info().lamports.borrow_mut());
    

        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_058_lam = dest_058_lam.checked_add(*acc_058_lam).unwrap();
        *acc_058_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct058<'info> {
    #[account(mut)]
    pub acc_058: AccountInfo<'info>,
    #[account(mut)]
    pub dest_058: AccountInfo<'info>,
}
