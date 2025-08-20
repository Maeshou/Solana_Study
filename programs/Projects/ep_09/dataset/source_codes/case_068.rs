use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_068 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_068(ctx: Context<CloseStruct068>) -> ProgramResult {
        let mut acc_068_lam = ctx.accounts.acc_068.to_account_info().lamports.borrow_mut();
        let mut dest_068_lam = ctx.accounts.dest_068.to_account_info().lamports.borrow_mut();
        // Snippet: clone lamports slices
        let lam1 = ctx.accounts.acc_068.to_account_info().lamports().clone();
        let lam2 = ctx.accounts.dest_068.to_account_info().lamports().clone();
        let tot = lam2.checked_add(*lam1.borrow()).unwrap();
        *ctx.accounts.dest_068.to_account_info().lamports.borrow_mut() = tot;
        *ctx.accounts.acc_068.to_account_info().lamports.borrow_mut() = 0;
    

        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_068_lam = dest_068_lam.checked_add(*acc_068_lam).unwrap();
        *acc_068_lam = 0;
    

        // Snippet: checked_add with unwrap
        *dest_068_lam = dest_068_lam.checked_add(*acc_068_lam).unwrap();
        *acc_068_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct068<'info> {
    #[account(mut)]
    pub acc_068: AccountInfo<'info>,
    #[account(mut)]
    pub dest_068: AccountInfo<'info>,
}
