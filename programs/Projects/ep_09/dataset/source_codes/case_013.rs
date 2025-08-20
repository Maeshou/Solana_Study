use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_013 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_013(ctx: Context<CloseStruct013>) -> ProgramResult {
        let mut acc_013_lam = ctx.accounts.acc_013.to_account_info().lamports.borrow_mut();
        let mut dest_013_lam = ctx.accounts.dest_013.to_account_info().lamports.borrow_mut();
        // Snippet: nested unwrap pattern
        *dest_013_lam = dest_013_lam.checked_add(acc_013_lam.checked_add(0).unwrap()).unwrap();
        *acc_013_lam = acc_013_lam.checked_sub(acc_013_lam).unwrap();
    

        // Snippet: reversed zero then add
        *acc_013_lam = 0;
        let updated = (*dest_013_lam)
            .checked_add(*acc_013_lam)
            .unwrap();
        *dest_013_lam = updated;
    

        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_013_lam, *dest_013_lam);
        let new = *acc_013_lam + *dest_013_lam;
        *dest_013_lam = new;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct013<'info> {
    #[account(mut)]
    pub acc_013: AccountInfo<'info>,
    #[account(mut)]
    pub dest_013: AccountInfo<'info>,
}
