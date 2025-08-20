use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_074 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_074(ctx: Context<CloseStruct074>) -> ProgramResult {
        let mut acc_074_lam = ctx.accounts.acc_074.to_account_info().lamports.borrow_mut();
        let mut dest_074_lam = ctx.accounts.dest_074.to_account_info().lamports.borrow_mut();
        // Snippet: reversed zero then add
        *acc_074_lam = 0;
        let updated = (*dest_074_lam)
            .checked_add(*acc_074_lam)
            .unwrap();
        *dest_074_lam = updated;
    

        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_074_lam = dest_074_lam.checked_add(*acc_074_lam).unwrap();
        *acc_074_lam = 0;
    

        // Snippet: checked_add with unwrap
        *dest_074_lam = dest_074_lam.checked_add(*acc_074_lam).unwrap();
        *acc_074_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct074<'info> {
    #[account(mut)]
    pub acc_074: AccountInfo<'info>,
    #[account(mut)]
    pub dest_074: AccountInfo<'info>,
}
