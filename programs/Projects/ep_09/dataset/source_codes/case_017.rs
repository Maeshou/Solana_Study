use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_017 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_017(ctx: Context<CloseStruct017>) -> ProgramResult {
        let mut acc_017_lam = ctx.accounts.acc_017.to_account_info().lamports.borrow_mut();
        let mut dest_017_lam = ctx.accounts.dest_017.to_account_info().lamports.borrow_mut();
        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_017_lam = dest_017_lam.checked_add(*acc_017_lam).unwrap();
        *acc_017_lam = 0;
    

        // Snippet: unwrap_or_default fallback
        let new_dst = dest_017_lam.checked_add(*acc_017_lam).unwrap_or_default();
        *dest_017_lam = new_dst;
        *acc_017_lam = 0;
    

        // Snippet: nested unwrap pattern
        *dest_017_lam = dest_017_lam.checked_add(acc_017_lam.checked_add(0).unwrap()).unwrap();
        *acc_017_lam = acc_017_lam.checked_sub(acc_017_lam).unwrap();
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct017<'info> {
    #[account(mut)]
    pub acc_017: AccountInfo<'info>,
    #[account(mut)]
    pub dest_017: AccountInfo<'info>,
}
