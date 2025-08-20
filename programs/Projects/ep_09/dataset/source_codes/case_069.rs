use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

#[program]
pub mod revival_vuln_069 {
    use super::*;

    // Vulnerable: allows revival within same transaction
    pub fn close_069(ctx: Context<CloseStruct069>) -> ProgramResult {
        let mut acc_069_lam = ctx.accounts.acc_069.to_account_info().lamports.borrow_mut();
        let mut dest_069_lam = ctx.accounts.dest_069.to_account_info().lamports.borrow_mut();
        // Snippet: plain add and zero
        let sum = *acc_069_lam + *dest_069_lam;
        *dest_069_lam = sum;
        *acc_069_lam = 0;
    

        // Snippet: checked_add with unwrap
        *dest_069_lam = dest_069_lam.checked_add(*acc_069_lam).unwrap();
        *acc_069_lam = 0;
    

        // Snippet: log before/after
        msg!("SRC={} DST={}", *acc_069_lam, *dest_069_lam);
        let new = *acc_069_lam + *dest_069_lam;
        *dest_069_lam = new;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CloseStruct069<'info> {
    #[account(mut)]
    pub acc_069: AccountInfo<'info>,
    #[account(mut)]
    pub dest_069: AccountInfo<'info>,
}
