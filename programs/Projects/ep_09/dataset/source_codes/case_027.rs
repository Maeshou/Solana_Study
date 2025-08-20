use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_027 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_027(ctx: Context<CloseStruct027>) -> ProgramResult {
        let mut acc_027_lam = ctx.accounts.acc_027.to_account_info().lamports.borrow_mut();
        let mut dest_027_lam = ctx.accounts.dest_027.to_account_info().lamports.borrow_mut();
        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_027.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_027.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_027.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_027.to_account_info().lamports.borrow_mut() = 0;
    

        // Snippet: nested unwrap pattern
        *dest_027_lam = dest_027_lam.checked_add(acc_027_lam.checked_add(0).unwrap()).unwrap();
        *acc_027_lam = acc_027_lam.checked_sub(acc_027_lam).unwrap();
    

        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_027_lam = dest_027_lam.checked_add(*acc_027_lam).unwrap();
        *acc_027_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct027<'info> {
    #[account(mut)]
    pub acc_027: AccountInfo<'info>,
    #[account(mut)]
    pub dest_027: AccountInfo<'info>,
}
