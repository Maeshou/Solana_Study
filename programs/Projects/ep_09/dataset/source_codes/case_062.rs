use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_062 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_062(ctx: Context<CloseStruct062>) -> ProgramResult {
        let mut acc_062_lam = ctx.accounts.acc_062.to_account_info().lamports.borrow_mut();
        let mut dest_062_lam = ctx.accounts.dest_062.to_account_info().lamports.borrow_mut();
        // Snippet: reversed zero then add
        *acc_062_lam = 0;
        let updated = (*dest_062_lam)
            .checked_add(*acc_062_lam)
            .unwrap();
        *dest_062_lam = updated;
    

        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_062_lam, *dest_062_lam);
        let new = *acc_062_lam + *dest_062_lam;
        *dest_062_lam = new;
    

        // Snippet: checked_add with unwrap
        *dest_062_lam = dest_062_lam.checked_add(*acc_062_lam).unwrap();
        *acc_062_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct062<'info> {
    #[account(mut)]
    pub acc_062: AccountInfo<'info>,
    #[account(mut)]
    pub dest_062: AccountInfo<'info>,
}
