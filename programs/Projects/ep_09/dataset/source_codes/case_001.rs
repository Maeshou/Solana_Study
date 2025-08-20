use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_001 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_001(ctx: Context<CloseStruct001>) -> ProgramResult {
        let mut acc_001_lam = ctx.accounts.acc_001.to_account_info().lamports.borrow_mut();
        let mut dest_001_lam = ctx.accounts.dest_001.to_account_info().lamports.borrow_mut();
        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_001_lam, *dest_001_lam);
        let new = *acc_001_lam + *dest_001_lam;
        *dest_001_lam = new;
    

        // Snippet: checked_add with unwrap
        *dest_001_lam = dest_001_lam.checked_add(*acc_001_lam).unwrap();
        *acc_001_lam = 0;
    

        // Snippet: nested unwrap pattern
        *dest_001_lam = dest_001_lam.checked_add(acc_001_lam.checked_add(0).unwrap()).unwrap();
        *acc_001_lam = acc_001_lam.checked_sub(acc_001_lam).unwrap();
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct001<'info> {
    #[account(mut)]
    pub acc_001: AccountInfo<'info>,
    #[account(mut)]
    pub dest_001: AccountInfo<'info>,
}
