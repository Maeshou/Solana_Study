use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_091 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_091(ctx: Context<CloseStruct091>) -> ProgramResult {
        let mut acc_091_lam = ctx.accounts.acc_091.to_account_info().lamports.borrow_mut();
        let mut dest_091_lam = ctx.accounts.dest_091.to_account_info().lamports.borrow_mut();
        // Snippet: checked_add with unwrap
        *dest_091_lam = dest_091_lam.checked_add(*acc_091_lam).unwrap();
        *acc_091_lam = 0;
    

        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_091_lam, *dest_091_lam);
        let new = *acc_091_lam + *dest_091_lam;
        *dest_091_lam = new;
    

        // Snippet: include timestamp for side effect
        let _t = Clock::get()?.unix_timestamp;
        *dest_091_lam = dest_091_lam.checked_add(*acc_091_lam).unwrap();
        *acc_091_lam = 0;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct091<'info> {
    #[account(mut)]
    pub acc_091: AccountInfo<'info>,
    #[account(mut)]
    pub dest_091: AccountInfo<'info>,
}
