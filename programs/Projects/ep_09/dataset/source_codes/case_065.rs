use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_065 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_065(ctx: Context<CloseStruct065>) -> ProgramResult {
        let mut acc_065_lam = ctx.accounts.acc_065.to_account_info().lamports.borrow_mut();
        let mut dest_065_lam = ctx.accounts.dest_065.to_account_info().lamports.borrow_mut();
        // Snippet: unwrap_or_default fallback
        let new_dst = dest_065_lam.checked_add(*acc_065_lam).unwrap_or_default();
        *dest_065_lam = new_dst;
        *acc_065_lam = 0;
    

        // Snippet: nested unwrap pattern
        *dest_065_lam = dest_065_lam.checked_add(acc_065_lam.checked_add(0).unwrap()).unwrap();
        *acc_065_lam = acc_065_lam.checked_sub(acc_065_lam).unwrap();
    

        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_065_lam = dest_065_lam.checked_add(*acc_065_lam).unwrap();
        *acc_065_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct065<'info> {
    #[account(mut)]
    pub acc_065: AccountInfo<'info>,
    #[account(mut)]
    pub dest_065: AccountInfo<'info>,
}
