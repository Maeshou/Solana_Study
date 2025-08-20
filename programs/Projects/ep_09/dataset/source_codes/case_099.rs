use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_099 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_099(ctx: Context<CloseStruct099>) -> ProgramResult {
        let mut acc_099_lam = ctx.accounts.acc_099.to_account_info().lamports.borrow_mut();
        let mut dest_099_lam = ctx.accounts.dest_099.to_account_info().lamports.borrow_mut();
        // Snippet: unwrap_or_default fallback
        let new_dst = dest_099_lam.checked_add(*acc_099_lam).unwrap_or_default();
        *dest_099_lam = new_dst;
        *acc_099_lam = 0;
    

        // Snippet: reversed zero then add
        *acc_099_lam = 0;
        let updated = (*dest_099_lam)
            .checked_add(*acc_099_lam)
            .unwrap();
        *dest_099_lam = updated;
    

        // Snippet: helper function abstraction
        fn transfer(src: &mut u64, dst: &mut u64) {
            let tmp = **src;
            **src = 0;
            **dst = dst.checked_add(tmp).unwrap();
        }
        transfer(&mut ctx.accounts.acc_099.to_account_info().lamports.borrow_mut(),
                 &mut ctx.accounts.dest_099.to_account_info().lamports.borrow_mut());
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct099<'info> {
    #[account(mut)]
    pub acc_099: AccountInfo<'info>,
    #[account(mut)]
    pub dest_099: AccountInfo<'info>,
}
