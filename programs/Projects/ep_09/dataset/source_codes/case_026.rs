use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_026 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_026(ctx: Context<CloseStruct026>) -> ProgramResult {
        let mut acc_026_lam = ctx.accounts.acc_026.to_account_info().lamports.borrow_mut();
        let mut dest_026_lam = ctx.accounts.dest_026.to_account_info().lamports.borrow_mut();
        // Snippet: helper function abstraction
        fn transfer(src: &mut u64, dst: &mut u64) {
            let tmp = **src;
            **src = 0;
            **dst = dst.checked_add(tmp).unwrap();
        }
        transfer(&mut ctx.accounts.acc_026.to_account_info().lamports.borrow_mut(),
                 &mut ctx.accounts.dest_026.to_account_info().lamports.borrow_mut());
    

        // Snippet: reversed zero then add
        *acc_026_lam = 0;
        let updated = (*dest_026_lam)
            .checked_add(*acc_026_lam)
            .unwrap();
        *dest_026_lam = updated;
    

        // Snippet: unwrap_or_default fallback
        let new_dst = dest_026_lam.checked_add(*acc_026_lam).unwrap_or_default();
        *dest_026_lam = new_dst;
        *acc_026_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct026<'info> {
    #[account(mut)]
    pub acc_026: AccountInfo<'info>,
    #[account(mut)]
    pub dest_026: AccountInfo<'info>,
}
