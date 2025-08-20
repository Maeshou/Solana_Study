use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_060 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_060(ctx: Context<CloseStruct060>) -> ProgramResult {
        let mut acc_060_lam = ctx.accounts.acc_060.to_account_info().lamports.borrow_mut();
        let mut dest_060_lam = ctx.accounts.dest_060.to_account_info().lamports.borrow_mut();
        // Snippet: nested unwrap pattern
        *dest_060_lam = dest_060_lam.checked_add(acc_060_lam.checked_add(0).unwrap()).unwrap();
        *acc_060_lam = acc_060_lam.checked_sub(acc_060_lam).unwrap();
    

        // Snippet: reversed zero then add
        *acc_060_lam = 0;
        let updated = (*dest_060_lam)
            .checked_add(*acc_060_lam)
            .unwrap();
        *dest_060_lam = updated;
    

        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_060_lam, *dest_060_lam);
        let new = *acc_060_lam + *dest_060_lam;
        *dest_060_lam = new;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct060<'info> {
    #[account(mut)]
    pub acc_060: AccountInfo<'info>,
    #[account(mut)]
    pub dest_060: AccountInfo<'info>,
}
