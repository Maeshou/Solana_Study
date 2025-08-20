use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_094 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_094(ctx: Context<CloseStruct094>) -> ProgramResult {
        let mut acc_094_lam = ctx.accounts.acc_094.to_account_info().lamports.borrow_mut();
        let mut dest_094_lam = ctx.accounts.dest_094.to_account_info().lamports.borrow_mut();
        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_094_lam = dest_094_lam.checked_add(*acc_094_lam).unwrap();
        *acc_094_lam = 0;
    

        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_094_lam, *dest_094_lam);
        let new = *acc_094_lam + *dest_094_lam;
        *dest_094_lam = new;
    

        // Snippet: checked_add with unwrap
        *dest_094_lam = dest_094_lam.checked_add(*acc_094_lam).unwrap();
        *acc_094_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct094<'info> {
    #[account(mut)]
    pub acc_094: AccountInfo<'info>,
    #[account(mut)]
    pub dest_094: AccountInfo<'info>,
}
