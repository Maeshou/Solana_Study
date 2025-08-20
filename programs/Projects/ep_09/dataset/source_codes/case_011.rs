use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_011 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_011(ctx: Context<CloseStruct011>) -> ProgramResult {
        let mut acc_011_lam = ctx.accounts.acc_011.to_account_info().lamports.borrow_mut();
        let mut dest_011_lam = ctx.accounts.dest_011.to_account_info().lamports.borrow_mut();
        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_011.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_011.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_011.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_011.to_account_info().lamports.borrow_mut() = 0;
    

        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_011_lam = dest_011_lam.checked_add(*acc_011_lam).unwrap();
        *acc_011_lam = 0;
    

        // Snippet: reversed zero then add
        *acc_011_lam = 0;
        let updated = (*dest_011_lam)
            .checked_add(*acc_011_lam)
            .unwrap();
        *dest_011_lam = updated;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct011<'info> {
    #[account(mut)]
    pub acc_011: AccountInfo<'info>,
    #[account(mut)]
    pub dest_011: AccountInfo<'info>,
}
