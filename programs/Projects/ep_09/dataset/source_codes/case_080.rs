use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_080 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_080(ctx: Context<CloseStruct080>) -> ProgramResult {
        let mut acc_080_lam = ctx.accounts.acc_080.to_account_info().lamports.borrow_mut();
        let mut dest_080_lam = ctx.accounts.dest_080.to_account_info().lamports.borrow_mut();
        // Snippet: nested unwrap pattern
        *dest_080_lam = dest_080_lam.checked_add(acc_080_lam.checked_add(0).unwrap()).unwrap();
        *acc_080_lam = acc_080_lam.checked_sub(acc_080_lam).unwrap();
    

        // Snippet: unwrap_or_default fallback
        let new_dst = dest_080_lam.checked_add(*acc_080_lam).unwrap_or_default();
        *dest_080_lam = new_dst;
        *acc_080_lam = 0;
    

        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_080_lam = dest_080_lam.checked_add(*acc_080_lam).unwrap();
        *acc_080_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct080<'info> {
    #[account(mut)]
    pub acc_080: AccountInfo<'info>,
    #[account(mut)]
    pub dest_080: AccountInfo<'info>,
}
