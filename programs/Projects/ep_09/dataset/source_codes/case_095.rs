use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_095 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_095(ctx: Context<CloseStruct095>) -> ProgramResult {
        let mut acc_095_lam = ctx.accounts.acc_095.to_account_info().lamports.borrow_mut();
        let mut dest_095_lam = ctx.accounts.dest_095.to_account_info().lamports.borrow_mut();
        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_095_lam = dest_095_lam.checked_add(*acc_095_lam).unwrap();
        *acc_095_lam = 0;
    

        // Snippet: reversed zero then add
        *acc_095_lam = 0;
        let updated = (*dest_095_lam)
            .checked_add(*acc_095_lam)
            .unwrap();
        *dest_095_lam = updated;
    

        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_095_lam, *dest_095_lam);
        let new = *acc_095_lam + *dest_095_lam;
        *dest_095_lam = new;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct095<'info> {
    #[account(mut)]
    pub acc_095: AccountInfo<'info>,
    #[account(mut)]
    pub dest_095: AccountInfo<'info>,
}
