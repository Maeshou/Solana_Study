use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_092 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_092(ctx: Context<CloseStruct092>) -> ProgramResult {
        let mut acc_092_lam = ctx.accounts.acc_092.to_account_info().lamports.borrow_mut();
        let mut dest_092_lam = ctx.accounts.dest_092.to_account_info().lamports.borrow_mut();
        // Snippet: unwrap_or_default fallback
        let new_dst = dest_092_lam.checked_add(*acc_092_lam).unwrap_or_default();
        *dest_092_lam = new_dst;
        *acc_092_lam = 0;
    

        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_092_lam = dest_092_lam.checked_add(*acc_092_lam).unwrap();
        *acc_092_lam = 0;
    

        // Snippet: nested unwrap pattern
        *dest_092_lam = dest_092_lam.checked_add(acc_092_lam.checked_add(0).unwrap()).unwrap();
        *acc_092_lam = acc_092_lam.checked_sub(acc_092_lam).unwrap();
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct092<'info> {
    #[account(mut)]
    pub acc_092: AccountInfo<'info>,
    #[account(mut)]
    pub dest_092: AccountInfo<'info>,
}
