use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_030 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_030(ctx: Context<CloseStruct030>) -> ProgramResult {
        let mut acc_030_lam = ctx.accounts.acc_030.to_account_info().lamports.borrow_mut();
        let mut dest_030_lam = ctx.accounts.dest_030.to_account_info().lamports.borrow_mut();
        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_030_lam = dest_030_lam.checked_add(*acc_030_lam).unwrap();
        *acc_030_lam = 0;
    

        // Snippet: nested unwrap pattern
        *dest_030_lam = dest_030_lam.checked_add(acc_030_lam.checked_add(0).unwrap()).unwrap();
        *acc_030_lam = acc_030_lam.checked_sub(acc_030_lam).unwrap();
    

        // Snippet: plain add and zero
        let sum = *acc_030_lam + *dest_030_lam;
        *dest_030_lam = sum;
        *acc_030_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct030<'info> {
    #[account(mut)]
    pub acc_030: AccountInfo<'info>,
    #[account(mut)]
    pub dest_030: AccountInfo<'info>,
}
